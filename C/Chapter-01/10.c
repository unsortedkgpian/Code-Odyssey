#include<stdio.h>

main(){
  int c;
  while((c=getchar()) != EOF){
    if(c=='\t'){
      printf("\\t");
      continue;
    }
    else if (c=='\b'){
      printf("\\b");
      continue;
    }
    else if (c=='\\'){
      printf("\\\\");
      continue;
    }
    putchar(c);
  }
}
