const { DebugSession, InitializedEvent, TerminatedEvent, StoppedEvent, OutputEvent, Thread, StackFrame, Source, Scope, Variable } = require('@vscode/debugadapter');
const { DebugProtocol } = require('@vscode/debugprotocol');
const { Subject } = require('events');
const path = require('path');
const net = require('net');

/**
 * Debug adapter for LLM.lang
 */
class LLMLangDebugSession extends DebugSession {
    /**
     * Create a new debug session
     */
    constructor() {
        super();
        
        // Initialize the debug session
        this.setDebuggerLinesStartAt1(true);
        this.setDebuggerColumnsStartAt1(true);
        
        // Create a client for the debug adapter
        this.client = null;
        
        // Create a message queue
        this.messageQueue = [];
        this.messageId = 1;
        this.pendingRequests = new Map();
    }
    
    /**
     * Initialize the debug session
     * @param {DebugProtocol.InitializeRequestArguments} args
     */
    initializeRequest(response, args) {
        // Send the initialized event
        this.sendEvent(new InitializedEvent());
        
        // Set the capabilities
        response.body = {
            supportsConfigurationDoneRequest: true,
            supportsEvaluateForHovers: true,
            supportsStepBack: false,
            supportsSetVariable: true,
            supportsRestartFrame: false,
            supportsGotoTargetsRequest: false,
            supportsStepInTargetsRequest: false,
            supportsCompletionsRequest: true,
            supportsModulesRequest: false,
            supportsValueFormattingOptions: true,
            supportsExceptionInfoRequest: true,
            supportTerminateDebuggee: true,
            supportsDelayedStackTraceLoading: true,
            supportsLogPoints: true,
            supportsConditionalBreakpoints: true,
            supportsHitConditionalBreakpoints: false,
            supportsSetExpression: true,
            supportsTerminateRequest: true,
            supportsDataBreakpoints: false,
            supportsReadMemoryRequest: false,
            supportsDisassembleRequest: false,
            supportsCancelRequest: true,
            supportsBreakpointLocationsRequest: true,
            supportsClipboardContext: false,
            supportsSteppingGranularity: false,
            supportsInstructionBreakpoints: false,
            supportsExceptionFilterOptions: false,
        };
        
        // Send the response
        this.sendResponse(response);
    }
    
    /**
     * Launch the debugger
     * @param {DebugProtocol.LaunchRequestArguments} args
     */
    async launchRequest(response, args) {
        // Connect to the debug adapter
        try {
            await this.connectToDebugAdapter();
            
            // Send the launch request to the debug adapter
            const result = await this.sendRequest('launch', args);
            
            // Send the response
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Set breakpoints
     * @param {DebugProtocol.SetBreakpointsRequestArguments} args
     */
    async setBreakpointsRequest(response, args) {
        try {
            // Send the setBreakpoints request to the debug adapter
            const result = await this.sendRequest('setBreakpoints', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Configuration done
     */
    async configurationDoneRequest(response, args) {
        try {
            // Send the configurationDone request to the debug adapter
            const result = await this.sendRequest('configurationDone', args);
            
            // Send the response
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Continue execution
     */
    async continueRequest(response, args) {
        try {
            // Send the continue request to the debug adapter
            const result = await this.sendRequest('continue', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Step over
     */
    async nextRequest(response, args) {
        try {
            // Send the next request to the debug adapter
            const result = await this.sendRequest('next', args);
            
            // Send the response
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Step into
     */
    async stepInRequest(response, args) {
        try {
            // Send the stepIn request to the debug adapter
            const result = await this.sendRequest('stepIn', args);
            
            // Send the response
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Step out
     */
    async stepOutRequest(response, args) {
        try {
            // Send the stepOut request to the debug adapter
            const result = await this.sendRequest('stepOut', args);
            
            // Send the response
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Get threads
     */
    async threadsRequest(response) {
        try {
            // Send the threads request to the debug adapter
            const result = await this.sendRequest('threads', {});
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Get stack trace
     * @param {DebugProtocol.StackTraceRequestArguments} args
     */
    async stackTraceRequest(response, args) {
        try {
            // Send the stackTrace request to the debug adapter
            const result = await this.sendRequest('stackTrace', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Get scopes
     * @param {DebugProtocol.ScopesRequestArguments} args
     */
    async scopesRequest(response, args) {
        try {
            // Send the scopes request to the debug adapter
            const result = await this.sendRequest('scopes', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Get variables
     * @param {DebugProtocol.VariablesRequestArguments} args
     */
    async variablesRequest(response, args) {
        try {
            // Send the variables request to the debug adapter
            const result = await this.sendRequest('variables', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Evaluate an expression
     * @param {DebugProtocol.EvaluateRequestArguments} args
     */
    async evaluateRequest(response, args) {
        try {
            // Send the evaluate request to the debug adapter
            const result = await this.sendRequest('evaluate', args);
            
            // Send the response
            response.body = result;
            this.sendResponse(response);
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
        }
    }
    
    /**
     * Disconnect the debugger
     */
    async disconnectRequest(response, args) {
        try {
            // Send the disconnect request to the debug adapter
            const result = await this.sendRequest('disconnect', args);
            
            // Send the response
            this.sendResponse(response);
            
            // Close the client
            if (this.client) {
                this.client.end();
                this.client = null;
            }
        } catch (error) {
            // Send the error
            response.success = false;
            response.message = error.message;
            this.sendResponse(response);
            
            // Close the client
            if (this.client) {
                this.client.end();
                this.client = null;
            }
        }
    }
    
    /**
     * Connect to the debug adapter
     */
    connectToDebugAdapter() {
        return new Promise((resolve, reject) => {
            // Create a client
            this.client = net.createConnection({ port: 4711 }, () => {
                console.log('Connected to debug adapter');
                resolve();
            });
            
            // Handle data
            this.client.on('data', (data) => {
                // Parse the data
                const messages = this.parseMessages(data.toString());
                
                // Process the messages
                for (const message of messages) {
                    this.processMessage(message);
                }
            });
            
            // Handle errors
            this.client.on('error', (error) => {
                console.error(`Debug adapter error: ${error}`);
                reject(error);
            });
            
            // Handle close
            this.client.on('close', () => {
                console.log('Debug adapter connection closed');
                this.client = null;
                
                // Send the terminated event
                this.sendEvent(new TerminatedEvent());
            });
        });
    }
    
    /**
     * Send a request to the debug adapter
     * @param {string} command
     * @param {any} args
     */
    sendRequest(command, args) {
        return new Promise((resolve, reject) => {
            // Create a request
            const request = {
                seq: this.messageId++,
                type: 'request',
                command,
                arguments: args,
            };
            
            // Add the request to the pending requests
            this.pendingRequests.set(request.seq, { resolve, reject });
            
            // Send the request
            this.sendMessage(request);
        });
    }
    
    /**
     * Send a message to the debug adapter
     * @param {any} message
     */
    sendMessage(message) {
        // Convert the message to JSON
        const json = JSON.stringify(message);
        
        // Add the message to the queue
        this.messageQueue.push(`Content-Length: ${Buffer.byteLength(json, 'utf8')}\r\n\r\n${json}`);
        
        // Send the messages
        this.sendMessages();
    }
    
    /**
     * Send messages to the debug adapter
     */
    sendMessages() {
        // Check if we have a client
        if (!this.client) {
            return;
        }
        
        // Send the messages
        while (this.messageQueue.length > 0) {
            const message = this.messageQueue.shift();
            this.client.write(message);
        }
    }
    
    /**
     * Parse messages from the debug adapter
     * @param {string} data
     */
    parseMessages(data) {
        const messages = [];
        let position = 0;
        
        while (position < data.length) {
            // Find the content length
            const contentLengthMatch = data.substr(position).match(/Content-Length: (\d+)\r\n\r\n/);
            
            if (!contentLengthMatch) {
                break;
            }
            
            // Get the content length
            const contentLength = parseInt(contentLengthMatch[1], 10);
            
            // Get the content
            const contentStart = position + contentLengthMatch[0].length;
            const content = data.substr(contentStart, contentLength);
            
            // Parse the content
            try {
                const message = JSON.parse(content);
                messages.push(message);
            } catch (error) {
                console.error(`Failed to parse message: ${error}`);
            }
            
            // Move to the next message
            position = contentStart + contentLength;
        }
        
        return messages;
    }
    
    /**
     * Process a message from the debug adapter
     * @param {any} message
     */
    processMessage(message) {
        // Check the message type
        switch (message.type) {
            case 'response':
                // Get the pending request
                const pendingRequest = this.pendingRequests.get(message.request_seq);
                
                if (pendingRequest) {
                    // Remove the pending request
                    this.pendingRequests.delete(message.request_seq);
                    
                    // Check if the response was successful
                    if (message.success) {
                        // Resolve the promise
                        pendingRequest.resolve(message.body);
                    } else {
                        // Reject the promise
                        pendingRequest.reject(new Error(message.message));
                    }
                }
                break;
            
            case 'event':
                // Process the event
                switch (message.event) {
                    case 'initialized':
                        this.sendEvent(new InitializedEvent());
                        break;
                    
                    case 'stopped':
                        this.sendEvent(new StoppedEvent(message.body.reason, message.body.threadId));
                        break;
                    
                    case 'terminated':
                        this.sendEvent(new TerminatedEvent());
                        break;
                    
                    case 'output':
                        this.sendEvent(new OutputEvent(message.body.output, message.body.category));
                        break;
                }
                break;
        }
    }
}

// Start the debug session
DebugSession.run(LLMLangDebugSession);
