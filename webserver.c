/**
 * HTTP server in C
 * Serves a form for submitting messages and displays submission history
 * Handles both GET and POST requests on port 5000
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/types.h>
#include <signal.h>
#include <time.h>

#define PORT 5000
#define BUFFER_SIZE 4096
#define MAX_SUBMISSIONS 100
#define MAX_SUBMISSION_LENGTH 1024

// Structure to store submissions
typedef struct {
    char message[MAX_SUBMISSION_LENGTH];
    char timestamp[32];
} Submission;

// Global variables
int server_fd; // For cleanup in signal handler
Submission submissions[MAX_SUBMISSIONS]; // Store submission history
int submission_count = 0; // Track number of submissions

// Signal handler for graceful shutdown
void handle_signal(int sig) {
    printf("\nCaught signal %d. Shutting down server...\n", sig);
    if (server_fd > 0) {
        close(server_fd);
    }
    exit(0);
}

// Function to generate timestamp for submissions
void generate_timestamp(char *timestamp_buffer, size_t buffer_size) {
    time_t now = time(NULL);
    struct tm *t = localtime(&now);
    strftime(timestamp_buffer, buffer_size, "%Y-%m-%d %H:%M:%S", t);
}

// Add a new submission to the history
void add_submission(const char *message) {
    if (submission_count >= MAX_SUBMISSIONS) {
        // If we reach the limit, shift all entries to make room for new one
        for (int i = 0; i < MAX_SUBMISSIONS - 1; i++) {
            memcpy(&submissions[i], &submissions[i + 1], sizeof(Submission));
        }
        submission_count = MAX_SUBMISSIONS - 1;
    }
    
    // Add the new submission
    strncpy(submissions[submission_count].message, message, MAX_SUBMISSION_LENGTH - 1);
    submissions[submission_count].message[MAX_SUBMISSION_LENGTH - 1] = '\0'; // Ensure null termination
    
    // Add timestamp
    generate_timestamp(submissions[submission_count].timestamp, sizeof(submissions[submission_count].timestamp));
    
    submission_count++;
}

// Initialize with a welcome message
void init_submissions() {
    add_submission("Hello world!");
}

// Generate the HTML content for the page
void generate_html_response(char *response_buffer, size_t buffer_size) {
    // Start with the header
    int offset = snprintf(response_buffer, buffer_size,
        "<!DOCTYPE html>\n"
        "<html>\n"
        "<head>\n"
        "    <title>Message Board</title>\n"
        "    <style>\n"
        "        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n"
        "        .submission { border-bottom: 1px solid #eee; padding: 10px 0; }\n"
        "        .message { font-size: 16px; }\n"
        "        .timestamp { color: #888; font-size: 12px; }\n"
        "        form { margin-top: 20px; padding: 15px; background: #f8f8f8; border-radius: 5px; }\n"
        "        input[type=text] { width: 80%%; padding: 8px; }\n"
        "        input[type=submit] { padding: 8px 15px; background: #4CAF50; color: white; border: none; cursor: pointer; }\n"
        "    </style>\n"
        "</head>\n"
        "<body>\n"
        "    <h1>Message Board</h1>\n"
    );

    // Add the submissions history
    for (int i = 0; i < submission_count; i++) {
        offset += snprintf(response_buffer + offset, buffer_size - offset,
            "    <div class=\"submission\">\n"
            "        <div class=\"message\">%s</div>\n"
            "        <div class=\"timestamp\">%s</div>\n"
            "    </div>\n",
            submissions[i].message,
            submissions[i].timestamp
        );
    }

    // Add the submission form
    offset += snprintf(response_buffer + offset, buffer_size - offset,
        "    <form method=\"POST\" action=\"/\">\n"
        "        <input type=\"text\" name=\"message\" placeholder=\"Enter your message\" required>\n"
        "        <input type=\"submit\" value=\"Submit\">\n"
        "    </form>\n"
        "</body>\n"
        "</html>\n"
    );
}

// Parse POST request to extract the message
char* parse_post_data(char *buffer) {
    static char message[MAX_SUBMISSION_LENGTH];
    char *content_start = strstr(buffer, "\r\n\r\n");
    
    if (!content_start) {
        return NULL; // No content found
    }
    
    content_start += 4; // Skip over the \r\n\r\n
    
    // Look for the message parameter
    char *message_param = strstr(content_start, "message=");
    if (!message_param) {
        return NULL; // No message parameter found
    }
    
    message_param += 8; // Skip over "message="
    
    // Copy and decode the message
    int i = 0, j = 0;
    while (message_param[i] && message_param[i] != '&' && message_param[i] != '\r' && message_param[i] != '\n' && j < MAX_SUBMISSION_LENGTH - 1) {
        if (message_param[i] == '+') {
            message[j++] = ' '; // Replace '+' with space
        } else if (message_param[i] == '%' && message_param[i+1] && message_param[i+2]) {
            // Handle URL encoding (e.g., %20 for space)
            char hex[3] = {message_param[i+1], message_param[i+2], 0};
            int value;
            sscanf(hex, "%x", &value);
            message[j++] = (char)value;
            i += 2;
        } else {
            message[j++] = message_param[i];
        }
        i++;
    }
    message[j] = '\0'; // Null-terminate
    
    return message;
}

int main() {
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    int new_socket;
    char buffer[BUFFER_SIZE] = {0};
    char response[BUFFER_SIZE * 4]; // Larger buffer for HTML content
    
    // Initialize submission history
    init_submissions();
    
    // Setup signal handlers for graceful shutdown
    signal(SIGINT, handle_signal);
    signal(SIGTERM, handle_signal);
    
    // Create server socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }
    
    // Set socket options to reuse address and port
    int opt = 1;
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt))) {
        perror("Setsockopt failed");
        exit(EXIT_FAILURE);
    }
    
    // Configure server address
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY; // Bind to 0.0.0.0
    address.sin_port = htons(PORT);
    
    // Bind socket to port
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }
    
    // Listen for connections
    if (listen(server_fd, 10) < 0) {
        perror("Listen failed");
        exit(EXIT_FAILURE);
    }
    
    printf("Server started on port %d...\n", PORT);
    
    // Main server loop
    while (1) {
        printf("Waiting for a connection...\n");
        
        // Accept connection from client
        if ((new_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t*)&addrlen)) < 0) {
            perror("Accept failed");
            continue; // Continue to accept next connection
        }
        
        char client_ip[INET_ADDRSTRLEN];
        inet_ntop(AF_INET, &(address.sin_addr), client_ip, INET_ADDRSTRLEN);
        printf("Connection accepted from %s:%d\n", client_ip, ntohs(address.sin_port));
        
        // Read client request
        ssize_t bytes_read = read(new_socket, buffer, BUFFER_SIZE);
        if (bytes_read < 0) {
            perror("Read failed");
            close(new_socket);
            continue;
        }
        
        // Process the request based on method
        if (strncmp(buffer, "GET", 3) == 0) {
            printf("Received GET request\n");
            
            // Generate HTML for the page
            generate_html_response(response, sizeof(response));
            
            // Create HTTP headers
            char headers[BUFFER_SIZE];
            int content_length = strlen(response);
            snprintf(headers, BUFFER_SIZE,
                "HTTP/1.1 200 OK\r\n"
                "Content-Type: text/html\r\n"
                "Content-Length: %d\r\n"
                "Connection: close\r\n"
                "\r\n", content_length);
            
            // Send headers and content
            write(new_socket, headers, strlen(headers));
            write(new_socket, response, content_length);
            
            printf("GET response sent successfully\n");
            
        } else if (strncmp(buffer, "POST", 4) == 0) {
            printf("Received POST request\n");
            
            // Parse the POST data to get the message
            char *message = parse_post_data(buffer);
            if (message && strlen(message) > 0) {
                printf("Received message: %s\n", message);
                add_submission(message);
            }
            
            // Send a redirect to reload the page
            const char *redirect_response =
                "HTTP/1.1 303 See Other\r\n"
                "Location: /\r\n"
                "Connection: close\r\n"
                "\r\n";
            
            write(new_socket, redirect_response, strlen(redirect_response));
            printf("POST response (redirect) sent successfully\n");
            
        } else {
            // Method not supported
            const char *error_response = 
                "HTTP/1.1 501 Not Implemented\r\n"
                "Content-Type: text/plain\r\n"
                "Content-Length: 22\r\n"
                "Connection: close\r\n"
                "\r\n"
                "Method not supported.";
            
            write(new_socket, error_response, strlen(error_response));
            printf("Unsupported HTTP method\n");
        }
        
        // Close client socket
        close(new_socket);
        printf("Connection closed\n");
        
        // Clear buffer for next request
        memset(buffer, 0, BUFFER_SIZE);
    }
    
    // Should never reach here, but for completeness
    close(server_fd);
    return 0;
}
