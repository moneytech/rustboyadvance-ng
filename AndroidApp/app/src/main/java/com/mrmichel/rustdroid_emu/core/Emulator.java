package com.mrmichel.rustdroid_emu.core;

import com.mrmichel.rustboyadvance.EmulatorBindings;

public class Emulator {

    public class EmulatorException extends Exception {
        public EmulatorException(String errorMessage) {
            super(errorMessage);
        }
    }

    /// context received by the native binding
    private long ctx = -1;

    private int[] frameBuffer;
    public Keypad keypad;

    static {
        System.loadLibrary("rustboyadvance_jni");
    }

    public Emulator() {
        frameBuffer = new int[240 * 160];
        keypad = new Keypad();
    }

    public int[] getFrameBuffer() {
        return frameBuffer;
    }

    public synchronized void runFrame() throws EmulatorBindings.NativeBindingException {
        EmulatorBindings.setKeyState(this.ctx, this.keypad.getKeyState());
        EmulatorBindings.runFrame(this.ctx, this.frameBuffer);
    }

    public synchronized void setKeyState(int keyState) {
        EmulatorBindings.setKeyState(this.ctx, keyState);
    }


    public byte[] saveState() throws EmulatorBindings.NativeBindingException {
        return EmulatorBindings.saveState(this.ctx);
    }


    public void loadState(byte[] state) throws EmulatorBindings.NativeBindingException {
        EmulatorBindings.loadState(this.ctx, state);
    }


    public synchronized void open(byte[] bios, byte[] rom, String saveName) throws EmulatorBindings.NativeBindingException {
        this.ctx = EmulatorBindings.openEmulator(bios, rom, saveName);
    }

    public synchronized void close() {
        if (this.ctx != -1) {
            EmulatorBindings.closeEmulator(this.ctx);
            this.ctx = -1;

        }
    }

    public boolean isOpen() {
        return this.ctx != -1;
    }

    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        close();
    }

    public synchronized void log() {
        EmulatorBindings.log(this.ctx);
    }
}