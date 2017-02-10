package com.mackie.rustyjvm;

public class TestVM {
    public static native void nativeInt(int i);

    public static void simple() {
        int a = 1;
        nativeInt(a);
    }

    public static void staticcall() {
        int a = 1;
        a = staticMethod(a);
        nativeInt(a);
    }

    private static int staticMethod(int a) {
        nativeInt(a);
        a = a * 2;
        nativeInt(a);
        return a;
    }
}
