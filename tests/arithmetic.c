#include <stdio.h>
#include <inttypes.h>

int64_t f0();
int64_t e0();
int64_t f1();
int64_t e1();
int64_t f2();
int64_t e2();
int64_t f3();
int64_t e3();
int64_t f4();
int64_t e4();

int main()
{
    /* printf("%d\n", f0()); */
    /* printf("%d\n", f1()); */
    /* printf("%d\n", f2()); */
    /* printf("%d\n", f3()); */
    /* printf("%d\n", f4()); */
    int all_pass = 0;

    int64_t out = f0();
    int64_t expected = e0();
    if (out != expected) {
        printf("error: expected %d, got %d\n", out, expected);
        all_pass = 1;
    }

    int64_t out1 = f1();
    int64_t expected1 = e1();
    if (out1 != expected1) {
        printf("error: expected %d, got %d\n", out1, expected1);
        all_pass = 1;
    }

    int64_t out2 = f2();
    int64_t expected2 = e2();
    if (out2 != expected2) {
        printf("error: expected %d, got %d\n", out2, expected2);
        all_pass = 1;
    }

    int64_t out3 = f3();
    int64_t expected3 = e3();
    if (out3 != expected3) {
        printf("error: expected %d, got %d\n", out3, expected3);
        all_pass = 1;
    }

    int64_t out4 = f4();
    int64_t expected4 = e4();
    if (out4 != expected4) {
        printf("error: expected %d, got %d\n", out4, expected4);
        all_pass = 1;
    }

    return all_pass;
}
