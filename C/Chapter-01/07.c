#include<stdio.h>

int main(){
  long nc;
  int c;
  nc =0;
  while ((c=getchar()) !=EOF){
    //++nc;
    if(c == '\n') ++nc;
  }
  printf("%ld\n",nc);
  return 0;
}
