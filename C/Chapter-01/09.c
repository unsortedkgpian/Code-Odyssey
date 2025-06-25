#include<stdio.h>

main(){
  int c;
  int nb=0;
  while((c=getchar()) != EOF){
    if (c==' ' & nb>0){
      continue;
    }
    else if (nb==0 && c==' ')++nb;
    else nb=0;
    putchar(c);
  }
}
