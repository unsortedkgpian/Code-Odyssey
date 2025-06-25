#include<stdio.h>

#define LOWER 0
#define UPPER 3000000
#define STEP 1
/*for effective ways*/

int main(){
  for(int fahr=UPPER;fahr>=LOWER;fahr-=STEP){
    printf("%3d\t%.2f\n",fahr, 5.0*(fahr-32)/9.0);
  }
}
