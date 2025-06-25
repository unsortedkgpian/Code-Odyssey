#include<stdio.h>
// -> int c is used because it can handle the  EOF, -1 as it is end of file 
int main() {
  int c;
  c = getchar();
  while(c!=EOF){
    putchar(c);
    c = getchar();
  }
  printf("Is is completed\n");
}
