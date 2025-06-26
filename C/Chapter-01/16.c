#include<stdio.h>

float fahrtocelsius(float n);

int main(){
    
    for(float i=0;i<3000;++i){
        printf("%.2f /t %.2f\n", i, fahrtocelsius(i));
    }


    return 0;
}



float fahrtocelsius(float n){
    return 5.0*(n-32.0)/9.0;
}
