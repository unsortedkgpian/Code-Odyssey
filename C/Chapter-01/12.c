#include<stdio.h>

main(){
    int c;
    int nb=0;
    while((c=getchar()) !=EOF){
        if((c==' ' || c=='\t' || c=='\n') && nb>0) continue;
        else if (c==' ' || c=='\t' || c=='n'){
            ++nb;
            printf("\n");
            continue;
        }
        nb=0;
        putchar(c);
        
    }
}
