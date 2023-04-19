const int trig = 2;   
const int echo = 3;   
long tiempo; 
long distancia;
const float sonido = 34300.0; // Velocidad del sonido en cm/s

void setup() {
  Serial.begin(115200);//iniciailzamos la comunicación
  pinMode(trig, OUTPUT); //pin como salida
  pinMode(echo, INPUT);  //pin como entrada
  digitalWrite(trig, LOW);//Inicializamos el pin con 0
}

void loop()
{

  

  digitalWrite(trig, HIGH);
  delayMicroseconds(10);          
  digitalWrite(trig, LOW);
  
  tiempo = pulseIn(echo, HIGH); 
  distancia = tiempo * 0.000001 * sonido/2;             

  Serial.print(distancia);      

  delay(700);          
}
