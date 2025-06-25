#include<stdio.h>

int main(){
  long nb =0, nt =0, nl =0;
  int c;
  while((c=getchar()) !=EOF){
    if(c==' ') ++nb;
    else if (c=='\t') ++nt;
    else if(c=='\n') ++nl;
  }

  printf("No of blanks -> %ld\n", nb);
  printf("No of tabs -> %ld\n", nt);
  printf("No of new line -> %ld\n", nl);
}
