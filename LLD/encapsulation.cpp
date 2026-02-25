#include<iostream>
#include<string>

using namespace std;


class SportsCar {
// public:
private:
    //characters
    string brand;
    string model;
    bool isEngineOn;
    int currentSpeed;
    int currentGear;
    string tyre;

public:
    SportsCar(string b, string m){
        this->brand = b;
        this->model = m;
        isEngineOn = false;
        currentSpeed = 0;
        currentGear = 0;
        tyre = "MRF";
    }

// getters and seetters 
    int getCurrentSpeed(){
        return this->currentSpeed;
    }


    string getCurrentTyre(){
        return this->tyre;
    }

    void setCurrentTyre(string t){
        this->tyre = t;
    }

    // Methdo -> behavior
    void startEngine(){
        isEngineOn = true;
        cout<< brand<< " "<< model<<" :Engine starts with a roar!"<<endl;
    }

    void shiftGear(int gear){
        if(!isEngineOn){
            cout<< brand<< " "<<model<<" :Engine is off! Cannot Shift Gear."<<endl;
            return;
        }
        currentGear = gear;
        cout<< brand<<" "<<model<<": Shifted to gear "<<currentGear <<endl;
    }

    void accelerate(){
        if(!isEngineOn){
            cout<< brand <<" "<<model<<" : Engine is off! Cannot accelerate."<<endl;
            return;
        }
        currentSpeed +=20;
        cout<< brand <<" "<<model<<" : Accelerating to "<< currentSpeed <<" km/h"<<endl;
    }
    void brake(){
        currentSpeed -=20;
        if(currentSpeed < 0) currentSpeed = 0;
        cout<< brand <<" "<< model <<" : Braking! Speed is now "<<currentSpeed <<" km/h"<<endl;
    }

    void stopEngine(){
        isEngineOn = false;
        currentGear = 0;
        currentSpeed = 0;
        cout<< brand <<" "<< model<< " : Engine turnoff."<<endl;
    }

};

int main(){
    SportsCar* mySportsCar = new SportsCar("Ford", "Mustang");

    mySportsCar->startEngine();
    mySportsCar->shiftGear(1);
    mySportsCar->accelerate();
    mySportsCar->shiftGear(2);
    mySportsCar->accelerate();
    mySportsCar->brake();
    mySportsCar->stopEngine();

    // Setting aribitary value to speed
    // mySportsCar->currentSpeed = 500;
    // cout<<"Current Speed of My Sports Car is set to "<<mySportsCar->currentSpeed <<endl;


    cout<<"Current Speed of My Sports Car is set to "<<mySportsCar->getCurrentSpeed()<<endl;



    // Tyre
    cout<<"Current Tyre is : "<<mySportsCar->getCurrentTyre()<<endl;
    mySportsCar->setCurrentTyre("CEAT");
    cout<<"Current Tyre is : "<<mySportsCar->getCurrentTyre()<<endl;

    






    delete mySportsCar;



    return 0;
}
