int porcentaje;

void setup() {
  Serial.begin(115200);
}

void loop() {
  
  porcentaje = map(analogRead(0), 0, 1023, 0, 100);
  Serial.print(porcentaje);
  delay(700);
}
