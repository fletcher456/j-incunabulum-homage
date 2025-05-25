#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <ctype.h>

// Define a maximum message history size
#define MAX_MESSAGES 10
#define MAX_MESSAGE_LEN 1024
#define MAX_RESPONSE_SIZE 4096

// Use j_simple.c for our J interpreter implementation
extern char* execute_j_code(const char* code);

// Arrays to store message history
char messages[MAX_MESSAGES][MAX_MESSAGE_LEN];
int message_count = 0;

// HTML template for the response
const char* html_template = 
"HTTP/1.1 200 OK\r\n"
"Content-Type: text/html\r\n\r\n"
"<!DOCTYPE html>\n"
"<html>\n"
"<head>\n"
"    <title>J Language Interpreter</title>\n"
"    <style>\n"
"        body { font-family: Arial, sans-serif; line-height: 1.6; max-width: 800px; margin: 0 auto; padding: 20px; }\n"
"        h1 { color: #333; }\n"
"        form { margin-bottom: 20px; }\n"
"        input[type='text'] { width: 70%%; padding: 8px; }\n"
"        input[type='submit'] { padding: 8px 15px; background: #4CAF50; border: none; color: white; cursor: pointer; }\n"
"        .history { background: #f9f9f9; padding: 15px; border-radius: 5px; }\n"
"        .entry { margin-bottom: 10px; }\n"
"        .input { color: #0066cc; }\n"
"        .output { color: #cc6600; }\n"
"    </style>\n"
"</head>\n"
"<body>\n"
"    <h1>J Language Interpreter</h1>\n"
"    <p>Enter a J expression to evaluate:</p>\n"
"    <form method='post'>\n"
"        <input type='text' name='message' placeholder='Enter J expression (e.g., i.5 or 2+2)' required>\n"
"        <input type='submit' value='Evaluate'>\n"
"    </form>\n"
"    <div class='history'>\n"
"        <h2>Evaluation History</h2>\n"
"        %s\n"
"    </div>\n"
"</body>\n"
"</html>";

// Function to URL decode input
void url_decode(char *dst, const char *src) {
    char a, b;
    while (*src) {
        if ((*src == '%') && ((a = src[1]) && (b = src[2])) && 
            (isxdigit(a) && isxdigit(b))) {
            if (a >= 'a') a -= 'a' - 'A';
            if (a >= 'A') a -= ('A' - 10);
            else a -= '0';
            
            if (b >= 'a') b -= 'a' - 'A';
            if (b >= 'A') b -= ('A' - 10);
            else b -= '0';
            
            *dst++ = 16 * a + b;
            src += 3;
        } else if (*src == '+') {
            *dst++ = ' ';
            src++;
        } else {
            *dst++ = *src++;
        }
    }
    *dst = '\0';
}

// Function to add a message to history
void add_message(const char* input, const char* output) {
    if (message_count >= MAX_MESSAGES) {
        // Shift messages to make room for new one
        for (int i = 0; i < MAX_MESSAGES - 1; i++) {
            strcpy(messages[i], messages[i + 1]);
        }
        message_count = MAX_MESSAGES - 1;
    }
    
    // Format and store the new message
    snprintf(messages[message_count], MAX_MESSAGE_LEN, 
             "<div class='entry'><span class='input'>Input: %s</span><br>"
             "<span class='output'>Output: %s</span></div>", 
             input, output);
    
    message_count++;
}

// Function to generate the history HTML
void generate_history(char* buffer, size_t buffer_size) {
    buffer[0] = '\0';  // Initialize buffer to empty string
    
    for (int i = message_count - 1; i >= 0; i--) {
        strncat(buffer, messages[i], buffer_size - strlen(buffer) - 1);
    }
}

int main() {
    int server_fd, client_fd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_addr_len = sizeof(client_addr);
    char buffer[4096], response[MAX_RESPONSE_SIZE];
    char history_buffer[MAX_RESPONSE_SIZE];
    
    // Create socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }
    
    // Set socket options to reuse address
    int opt = 1;
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
        perror("Setsockopt failed");
        exit(EXIT_FAILURE);
    }
    
    // Prepare the sockaddr_in structure
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(5000);
    
    // Bind the socket
    if (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }
    
    // Listen for incoming connections
    if (listen(server_fd, 5) < 0) {
        perror("Listen failed");
        exit(EXIT_FAILURE);
    }
    
    printf("Server started on port 5000...\n");
    
    while (1) {
        printf("Waiting for a connection...\n");
        
        // Accept a connection
        if ((client_fd = accept(server_fd, (struct sockaddr *)&client_addr, &client_addr_len)) < 0) {
            perror("Accept failed");
            continue;
        }
        
        printf("Connection accepted from %s:%d\n", inet_ntoa(client_addr.sin_addr), ntohs(client_addr.sin_port));
        
        // Receive data from client
        int bytes_received = recv(client_fd, buffer, sizeof(buffer) - 1, 0);
        if (bytes_received < 0) {
            perror("Receive failed");
            close(client_fd);
            continue;
        }
        
        buffer[bytes_received] = '\0';
        
        // Check if it's a GET or POST request
        if (strncmp(buffer, "GET", 3) == 0) {
            printf("Received GET request\n");
            
            // Generate history HTML
            generate_history(history_buffer, sizeof(history_buffer));
            
            // Create response with history
            snprintf(response, sizeof(response), html_template, history_buffer);
            
            // Send response
            if (send(client_fd, response, strlen(response), 0) < 0) {
                perror("Send failed");
            } else {
                printf("GET response sent successfully\n");
            }
        } else if (strncmp(buffer, "POST", 4) == 0) {
            printf("Received POST request\n");
            
            // Find the start of the message data
            char* message_start = strstr(buffer, "message=");
            if (message_start) {
                message_start += 8;  // Skip "message="
                
                // Find the end of the message (either & or end of line)
                char* message_end = strstr(message_start, "&");
                if (!message_end) {
                    message_end = strstr(message_start, "\r\n");
                }
                
                if (message_end) {
                    *message_end = '\0';  // Null terminate the message
                    
                    // URL decode the message
                    char decoded_message[MAX_MESSAGE_LEN];
                    url_decode(decoded_message, message_start);
                    
                    // Process the J expression
                    char* result = execute_j_code(decoded_message);
                    
                    // Add to history
                    add_message(decoded_message, result);
                    
                    // Generate updated history HTML
                    generate_history(history_buffer, sizeof(history_buffer));
                    
                    // Create response with updated history
                    snprintf(response, sizeof(response), html_template, history_buffer);
                    
                    // Free the result string
                    free(result);
                    
                    // Send response with redirect
                    char redirect_response[MAX_RESPONSE_SIZE];
                    snprintf(redirect_response, sizeof(redirect_response),
                             "HTTP/1.1 303 See Other\r\nLocation: /\r\n\r\n");
                    
                    if (send(client_fd, redirect_response, strlen(redirect_response), 0) < 0) {
                        perror("Send failed");
                    } else {
                        printf("POST response (redirect) sent successfully\n");
                    }
                }
            }
        }
        
        // Close the connection
        close(client_fd);
        printf("Connection closed\n");
    }
    
    // Close the server socket
    close(server_fd);
    
    return 0;
}