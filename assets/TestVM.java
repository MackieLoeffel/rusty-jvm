package com.mackie.rustyjvm;

public class TestVM {
    public static native void nativeInt(int i);

    public static void simple() {
        int a = 1;
        nativeInt(a);
    }

}
