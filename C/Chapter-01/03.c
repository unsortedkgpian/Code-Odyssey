#include<stdio.h>

/*for effective ways*/

int main(){
  for(int fahr=0;fahr<=300000;fahr+=1){
    printf("%3d\t%.2f\n",fahr, 5.0*(fahr-32)/9.0);
  }
}
