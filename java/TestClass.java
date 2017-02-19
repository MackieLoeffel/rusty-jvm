package com.mackie.rustyjvm;

interface TestClassInterfaceA {}
interface TestClassInterfaceB extends TestClassInterfaceA {}
interface TestClassInterfaceC extends TestClassInterfaceB {}

class TestClassSuper implements TestClassInterfaceB {
    private int a;
    private long c;
    private byte d;
    private static long sl;

    public void virtualMethod() {}
}

public class TestClass extends TestClassSuper {
    public static void main(String[] args) throws Error {
        int a = 1 + 1;
    }

    private int a;
    private double d;
    private double[] e;
    private static short c;
    public void virtualMethod() {}
}
