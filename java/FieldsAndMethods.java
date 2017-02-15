

class FieldsAndMethodsSuper {
    protected int instanceA = 1;
    protected int instanceB = 200;
    protected short superC = 300;

    public FieldsAndMethodsSuper(int a) {
        instanceA = a * 100;
    }

    public void outputFields() {
        outputFieldsPrivate();
    }

    public final void outputFieldsFinal() {
        outputFieldsPrivate();
    }

    private final void outputFieldsPrivate() {
        FieldsAndMethods.dump_char3('S', ':', ' ');
        FieldsAndMethods.dump_longln(instanceA);
        FieldsAndMethods.dump_longln(instanceB);
        FieldsAndMethods.dump_longln(superC);
    }

    public void changeFields() {
        FieldsAndMethods.dump_char4('S', 'C', '!', '\n');
        instanceA *= 50;
        instanceB *= 50;
        superC *= 50;
    }

    public static long staticUpperMethod() {
        FieldsAndMethods.dump_char4('S', 'U', '!', '\n');
        return 100 * 22;
    }

    public FieldsAndMethodsSuper returnThis() {
        return this;
    }
}

public class FieldsAndMethods extends FieldsAndMethodsSuper {

    private int instanceA = 1;
    private long instanceB = 2;
    private short instanceC = 3;

    public FieldsAndMethods(int a) {
        super(a * 2);
        instanceA = a;
    }

    public void outputFields() {
        outputFieldsPrivate();
    }

    private void outputFieldsPrivate() {
        dump_char3('F', ':', ' ');
        dump_longln(instanceA);
        dump_longln(instanceB);
        dump_longln(instanceC);
        dump_longln(superC);
        dump_char4('F', 'S', ':', ' ');
        dump_longln(((FieldsAndMethodsSuper)this).instanceA);
        dump_longln(((FieldsAndMethodsSuper)this).instanceB);
        dump_longln(((FieldsAndMethodsSuper)this).superC);
    }

    public void changeFields() {
        dump_char4('F', 'C', '!', '\n');
        instanceA *= 2;
        instanceB *= 2;
        instanceC *= 2;
        superC *= 2;
    }

    public void printArray(int[] a) {
        dump_char3('A', ':', '\n');
        for(int i = 0; i < a.length; i++){
            dump_longln(a[i]);
        }
    }

    public void changeArray(int[] a) {
        dump_char3('L', ':', ' ');
        dump_longln(a.length);
        printArray(a);
        a[5] = 3;
        printArray(a);
    }

    public static void main(String[] args) {
        // test instance variables
        FieldsAndMethods m = new FieldsAndMethods(16);
        FieldsAndMethodsSuper s = new FieldsAndMethodsSuper(100);
        dump_longln(m.instanceA);
        dump_longln(m.instanceB);
        dump_longln(m.instanceC);
        dump_longln(m.superC);
        m.outputFields();
        m.outputFieldsFinal();
        m.outputFieldsPrivate();

        dump_longln(s.instanceA);
        dump_longln(s.instanceB);
        dump_longln(s.superC);
        s.outputFields();
        s.outputFieldsFinal();

        m.changeFields();

        m.outputFields();
        m.outputFieldsFinal();
        m.outputFieldsPrivate();

        s.changeFields();

        s.outputFields();
        s.outputFieldsFinal();

        s = m;

        dump_longln(s.instanceA);
        dump_longln(s.instanceB);
        dump_longln(s.superC);
        s.outputFields();
        s.outputFieldsFinal();

        s.changeFields();
        s.outputFields();
        s.outputFieldsFinal();

        m = (FieldsAndMethods)s;

        m.outputFields();
        m.outputFieldsFinal();
        m.outputFieldsPrivate();

        m.changeFields();

        m.outputFields();
        m.outputFieldsFinal();
        m.outputFieldsPrivate();

        dump_longln(FieldsAndMethodsSuper.staticUpperMethod());
        dump_longln(m.instanceA);
        dump_longln(m.returnThis().instanceA);

        int[] array = new int[]{1,2,3,4,5,6,7};
        m.printArray(array);
        m.changeArray(array);
        m.printArray(array);

        // TODO test static variables
        // of current class
        // of super class
        // of other class
        // of interface
        // with constant initialization
        // with static initialization method
        // TODO test interface methods
    }


    // output methods
    public static void dump_char(char c) {
        System.out.print(c);
    }
    public static void dump_char2(char c1, char c2) {
        dump_char(c1);
        dump_char(c2);
    }
    public static void dump_char3(char c1, char c2, char c3) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
    }
    public static void dump_char4(char c1, char c2, char c3, char c4) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
        dump_char(c4);
    }

    public static void dump_long_rec(long x) {
        if (x == 0) { return; }
        dump_long_rec(x / 10);
        dump_char((char)('0' + (x % 10)));
    }
    public static void dump_long(long x) {
        if (x == 0) {
            dump_char('0');
            return;
        }

        // special handling for minimal long value
        // because -Long.MIN_VALUE == Long.MIN_VALUE
        if(x == Long.MIN_VALUE) {
            dump_char2('-', '9');
            x = 223372036854775808L;
        }

        if(x < 0) {
            dump_char('-');
            x = -x;
        }
        dump_long_rec(x);
    }

    public static void dump_longln(long x) {
        dump_long(x);
        dump_char('\n');
    }
}
