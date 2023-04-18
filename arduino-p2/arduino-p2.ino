int randomNumber;


void setup() {
  Serial.begin(115200);
}

void loop() {

  randomNumber=random(100);
  Serial.print(randomNumber);
 
  delay(1500);
}
