//WORKING ARITHMETIC OPERATORS

#include <stdio.h>
int arithmetic_operators(){
    int val_1, val_2, output;
    printf("Enter first value: ");
    scanf("%d", &val_1);
    printf("Enter second value: ");
    scanf("%d", &val_2);

    output = val_1 + val_2;
    printf("Value 1 + Value 2 = %d \n",output);
    output = val_1 - val_2;
    printf("Value 1 - Value 2 = %d \n",output);
    output = val_1*val_2;
    printf("Value 1 * Value 2 = %d \n",output);
    output = val_1/val_2;
    printf("Value 1 / Value 2 = %d \n",output);
    output = val_1%val_2;
    printf("Remainder when Value 1 divided by Value 2 = %d \n", output);

    return 0;

}

// WORKING OF INCREMENT AND DECREMENT OPERATORS

int INCREMENT_AND_DECREMENT_OPERATORS(){
    int first_int, second_int;
    float first_float, second_float;
    printf("Enter first integer value: ");
    scanf("%d", &first_int);
    printf("Enter second integer value: ");
    scanf("%d", &second_int);
    printf("Enter first float value: ");
    scanf("%f", &first_float);
    printf("Enter second float value: ");
    scanf("%f", &second_float);

    printf("++first_integer = %d \n", ++first_int);
    printf("--second_integer = %d \n", --second_int);
    printf("++first_float = %f \n", ++first_float);
    printf("--second_float = %f \n", --second_float);

    return 0;
}

// WORKING OF ASSIGNMENT OPERATORS

int ASSIGNMENT_OPERATORS(){
    int grade, new_grade;
    printf("Enter grade: ");
    scanf("%d", &grade);

    new_grade = grade;
    printf("New grade = %d\n", new_grade);
    new_grade += grade;
    printf("New grade plus grade\n");
    printf("New grade = %d\n", new_grade);
    new_grade -= grade;
    printf("New grade minus grade\n");
    printf("New grade = %d\n", new_grade);
    new_grade *= grade;
    printf("New grade multiply grade\n");
    printf("New grade %d\n", new_grade);
    new_grade /= grade;
    printf("New grade divide grade\n");
    printf("New grade %d\n", new_grade);
    new_grade %= grade;
    printf("New grade mod grade\n");
    printf("New grade %d\n", new_grade);

    return 0;

}

// WORKING OF LOGICAL OPERATORS

int main(){
    int int_1, int_2, int_3, result;
    printf("Enter an integer number : ");
    scanf("%d", &int_1);
    printf("Enter another integer number : ");
    scanf("%d", &int_2);
    printf("Enter the last integer number : ");
    scanf("%d", &int_3);

    result = (int_1 == int_2) && (int_3>int_2);
    printf("(first_int == second_int)&&(third_int > second_int) is %d\n", result);
    result = (int_1==int_2)&&(int_3<int_2);
    printf("(first int == second int)&&(third int < second int) is %d\n", result);
    result = (int_1==int_2)||(int_3<int_2);
    printf("(first int == second int)||(third int < second int) is %d\n", result);
    result = (int_1!=int_2)||(int_3<int_2);
    printf("(first int != second int)||(third int < second int) is %d\n", result);
    result = !(int_1 != int_2);
    printf("!(first_int != second_int) is %d\n", result);
    result = !(int_1 == int_2);
    printf("!(first_int == second_int) is %d \n", result);

    return 0;
}