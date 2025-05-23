const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
const path = require('path');
const net = require('net');

let client;
let outputChannel;

/**
 * Activate the extension
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    console.log('Activating LLM.lang extension');
    
    // Create an output channel for the extension
    outputChannel = vscode.window.createOutputChannel('LLM.lang');
    outputChannel.appendLine('LLM.lang extension activated');
    
    // Register the restart command
    const restartCommand = vscode.commands.registerCommand('llm-lang.restart', () => {
        if (client) {
            client.stop();
            startLanguageServer(context);
        }
    });
    
    context.subscriptions.push(restartCommand);
    
    // Start the language server
    startLanguageServer(context);
    
    // Register the debug adapter factory
    const factory = new LLMLangDebugAdapterDescriptorFactory();
    context.subscriptions.push(vscode.debug.registerDebugAdapterDescriptorFactory('llm-lang', factory));
}

/**
 * Deactivate the extension
 */
function deactivate() {
    if (client) {
        return client.stop();
    }
    return undefined;
}

/**
 * Start the language server
 * @param {vscode.ExtensionContext} context
 */
function startLanguageServer(context) {
    // Get the path to the language server
    const config = vscode.workspace.getConfiguration('llm-lang');
    const lspPath = config.get('lspPath', 'llm_lang_lsp');
    
    // Log the language server path
    console.log(`Using language server at: ${lspPath}`);
    outputChannel.appendLine(`Using language server at: ${lspPath}`);
    
    // Create the server options
    const serverOptions = {
        run: {
            command: lspPath,
            transport: TransportKind.stdio,
        },
        debug: {
            command: lspPath,
            transport: TransportKind.stdio,
            options: {
                env: {
                    ...process.env,
                    RUST_LOG: 'debug',
                },
            },
        },
    };
    
    // Create the client options
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'llm' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.llm'),
        },
        outputChannel,
    };
    
    // Create the language client
    client = new LanguageClient(
        'llm-lang',
        'LLM.lang Language Server',
        serverOptions,
        clientOptions
    );
    
    // Start the client with error handling
    client.start().catch(error => {
        outputChannel.appendLine(`Failed to start language server: ${error.message}`);
        console.error(`Failed to start language server: ${error.message}`);
        
        // Show an error message to the user
        vscode.window.showErrorMessage(`Failed to start LLM.lang language server: ${error.message}. Check the output panel for more details.`);
    });
}

/**
 * Debug adapter descriptor factory for LLM.lang
 */
class LLMLangDebugAdapterDescriptorFactory {
    /**
     * Create a debug adapter descriptor
     * @param {vscode.DebugSession} session
     * @returns {vscode.ProviderResult<vscode.DebugAdapterDescriptor>}
     */
    createDebugAdapterDescriptor(session) {
        // Get the path to the debug adapter
        const config = vscode.workspace.getConfiguration('llm-lang');
        const dapPath = config.get('dapPath', 'llm_lang_dap');
        
        // Create a server on localhost:4711
        const server = net.createServer(socket => {
            socket.on('error', err => {
                console.error(`Socket error: ${err}`);
            });
            
            // Log the debug adapter path
            console.log(`Using debug adapter at: ${dapPath}`);
            outputChannel.appendLine(`Using debug adapter at: ${dapPath}`);
            
            // Connect to the debug adapter with error handling
            const dapProcess = require('child_process').spawn(dapPath, [], {
                shell: false,
                windowsVerbatimArguments: false
            });
            
            // Log stdout and stderr
            dapProcess.stdout.on('data', (data) => {
                console.log(`Debug adapter stdout: ${data}`);
                outputChannel.appendLine(`Debug adapter stdout: ${data}`);
            });
            
            dapProcess.stderr.on('data', (data) => {
                console.error(`Debug adapter stderr: ${data}`);
                outputChannel.appendLine(`Debug adapter stderr: ${data}`);
            });
            
            // Pipe the socket to the debug adapter
            dapProcess.stdout.pipe(socket);
            socket.pipe(dapProcess.stdin);
            
            // Handle errors
            dapProcess.on('error', err => {
                console.error(`Debug adapter error: ${err}`);
                socket.end();
            });
            
            // Handle exit
            dapProcess.on('exit', code => {
                console.log(`Debug adapter exited with code ${code}`);
                socket.end();
            });
            
            // Handle socket close
            socket.on('close', () => {
                dapProcess.kill();
            });
        });
        
        // Listen on localhost:4711
        server.listen(4711, () => {
            console.log('Debug adapter server listening on port 4711');
        });
        
        // Return a debug adapter descriptor
        return new vscode.DebugAdapterServer(4711);
    }
}

module.exports = {
    activate,
    deactivate,
};
