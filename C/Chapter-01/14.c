#include<stdio.h>

#define IN 1
#define OUT 0

main(){
    int c, state;
    state = OUT;

    int length[46];
    for(int i=0;i<46;++i)
        length[i]=0;

    int nw=0;
    int max =0;

    while((c=getchar()) != EOF){
        if(c==' ' || c=='\n' || c=='\t'){
            state = OUT;
            if(nw>0){
                ++length[nw];
                if(length[nw]>=max) max=length[nw];
            }
            nw=0;
        }else if (state == OUT){
            state =IN;
        }

        if(state == IN) {
            ++nw;
        }
    }

    // for(int i=1;i<46;++i){
    //     if(length[i] ==0) continue;
    //     printf("%d words \t", i);
    //     for(int j=0;j<length[i];j++){
    //         printf("* ");
    //     }
    //     printf("\n");
    // }
    for (int i=0;i<max;++i){
        for(int j=1;j<46;++j){
            if (length[j] !=0){
                if(i-max+length[j]>=0) {
                    printf("*");
                }
                printf("\t");
            }
        }
        printf("\n");
    }
    for(int i=0;i< 46;++i){
        if(length[i] !=0) {
            printf("%d-words\t", i);
        }
    }
}
