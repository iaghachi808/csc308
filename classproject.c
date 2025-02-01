#include <stdio.h>

int main(){

    char input;

    printf("Select your food item\n");
    printf("p for Poundo Yam/Edinkaiko soup  -N3,200\n");
    printf("f for fried rice & chicken       -N3,000\n");
    printf("a for amala and ewedu soup       -N2,500\n");
    printf("e for eba and egusi soup         -N2,000\n");
    printf("w for white rice & stew          -N2,500\n");
    printf("x to exit menu\n");
    scanf("%c", &input);

    switch(input){

        case 'x':
        printf("You have chose to exit\n");
        break;

        case 'p':
        printf("You have chosen Poundo yam\n");
        break;

        case 'f':
        printf("You have chosen fried rice & chicken\n");
        break;

        case 'a':
        printf("You have chosen amala and ewedu\n");
        break;

        case 'e':
        printf("You have chosen eba and egusi soup\n");
        break;

        case 'w':
        printf("You have chosen white rice & stew\n");
        break;
    }


    return 0;

}