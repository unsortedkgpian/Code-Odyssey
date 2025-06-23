#include<stdio.h>

int main(){
  float fahr, celsius;
  float lower, upper, step;

  lower = 0;
  upper =212;
  step = 1;

  fahr = lower;
  
  while(fahr <= upper){
    celsius = 5.0*(fahr -32 )/9.0;
    //printf("%f\t%f\n", fahr, celsius);
    printf("%6.2f\t%6.2f\n\n", fahr, celsius);
    fahr += step;
  }
}
