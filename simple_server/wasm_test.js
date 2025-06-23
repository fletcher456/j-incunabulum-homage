const fs = require('fs');
const path = require('path');

async function testWasm() {
    console.log('=== J Language WASM Test ===');
    
    // Check if we can load the module
    const wasmPath = './static/pkg/simple_server_bg.wasm';
    if (fs.existsSync(wasmPath)) {
        console.log('✅ WASM binary exists');
        const wasmBytes = fs.readFileSync(wasmPath);
        console.log(`WASM size: ${wasmBytes.length} bytes`);
        
        // Basic validation that it's a WASM file
        const wasmMagic = wasmBytes.subarray(0, 4);
        const expectedMagic = Buffer.from([0x00, 0x61, 0x73, 0x6d]);
        if (wasmMagic.equals(expectedMagic)) {
            console.log('✅ Valid WASM magic number detected');
        } else {
            console.log('❌ Invalid WASM magic number');
            console.log('Expected:', expectedMagic);
            console.log('Got:', wasmMagic);
        }
    } else {
        console.log('❌ WASM binary not found');
    }
    
    // Check JavaScript bindings
    const jsPath = './static/pkg/simple_server.js';
    if (fs.existsSync(jsPath)) {
        console.log('✅ JavaScript bindings exist');
        const jsContent = fs.readFileSync(jsPath, 'utf8');
        
        // Look for expected functions
        const functions = ['evaluate_j_expression', 'wasm_bindgen', 'init'];
        functions.forEach(func => {
            if (jsContent.includes(func)) {
                console.log(`✅ Found function: ${func}`);
            } else {
                console.log(`⚠️  Function not found: ${func}`);
            }
        });
    } else {
        console.log('❌ JavaScript bindings not found');
    }
}

testWasm().catch(console.error);