/**
 * Minimal HTTP server in C
 * Serves "Hello world!" text on port 5000
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

#define PORT 5000
#define BUFFER_SIZE 1024
#define CONTENT "Hello world!"

int server_fd; // Global for cleanup in signal handler

// Signal handler for graceful shutdown
void handle_signal(int sig) {
    printf("\nCaught signal %d. Shutting down server...\n", sig);
    if (server_fd > 0) {
        close(server_fd);
    }
    exit(0);
}

int main() {
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    int new_socket;
    char buffer[BUFFER_SIZE] = {0};
    
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
    
    // Prepare HTTP response with proper headers
    char *content = CONTENT;
    char response[BUFFER_SIZE];
    
    int content_length = strlen(content);
    snprintf(response, BUFFER_SIZE,
             "HTTP/1.1 200 OK\r\n"
             "Content-Type: text/plain\r\n"
             "Content-Length: %d\r\n"
             "Connection: close\r\n"
             "\r\n"
             "%s", 
             content_length, content);
    
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
        
        // Basic HTTP request parsing (just check if it's a GET)
        if (strncmp(buffer, "GET", 3) == 0) {
            printf("Received GET request\n");
            
            // Send HTTP response
            ssize_t bytes_sent = write(new_socket, response, strlen(response));
            if (bytes_sent < 0) {
                perror("Send failed");
            } else {
                printf("Response sent successfully (%zd bytes)\n", bytes_sent);
            }
        } else {
            // Only support GET for simplicity
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
