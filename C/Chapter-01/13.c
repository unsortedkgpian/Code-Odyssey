#include<stdio.h>

main(){
    int c, nwhite, nother;
    int ndigit[10];

    nwhite = nother =0;

    for(int i=0;i<10;++i)
        ndigit[i]=0;
    
    while((c=getchar()) != EOF)
        if(c >='0' && c<='9' ) ++ndigit[c-'0'];
        else if(c==' '||c=='\t'||c=='\n') ++nwhite;
        else ++nother;
    
    printf("digits =\n");
    for(int i=0;i<10;++i)
        printf("%d\t%d\n", i, ndigit[i]);
    
    printf("\n the white spaces are: %d \n others are : %d ", nwhite, nother);
}
