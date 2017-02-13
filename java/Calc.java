public class Calc {
    private static void dump_char(char c) {
        System.out.print(c);
    }
    private static void dump_char2(char c1, char c2) {
        dump_char(c1);
        dump_char(c2);
    }
    private static void dump_char3(char c1, char c2, char c3) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
    }
    private static void dump_char4(char c1, char c2, char c3, char c4) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
        dump_char(c4);
    }

    private static void dump_long_rec(long x) {
        if (x == 0) { return; }
        dump_long_rec(x / 10);
        dump_char((char)('0' + (x % 10)));
    }
    private static void dump_long(long x) {
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

    private static long sum(long n) {
        long s;
        long i;

        s = 0;
        for (i = 0; i <= n; i++) {
            s += i;
        }

        return s;
    }

    private static long fac(long n) {
        if (n == 0) {
            return 1;
        } else {
            return n * fac(n - 1);
        }
    }

    public static void main(String[] args) {
        long i;

        for (i = 0; i < 100; i++) {
            dump_char4('s', 'u', 'm', '(');
            dump_long(i);
            dump_char4(')', ' ', '=', ' ');
            dump_long(sum(i));
            dump_char('\n');

            dump_char4('f', 'a', 'c', '(');
            dump_long(i);
            dump_char4(')', ' ', '=', ' ');
            dump_long(fac(i));
            dump_char('\n');
        }

    }

}
