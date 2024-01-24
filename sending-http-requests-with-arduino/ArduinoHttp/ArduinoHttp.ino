#include <WiFiS3.h>
#include <ArduinoHttpClient.h>

const char SSID[] = "NETWORK_NAME";
const char PASS[] = "NETWORK_PASSWORD";
const char HOST_NAME[] = "echo.free.beeceptor.com";
const int HTTP_PORT = 443;

WiFiSSLClient wifi;
HttpClient client = HttpClient(wifi, HOST_NAME, HTTP_PORT);
int status = WL_IDLE_STATUS;

String PATH_NAME = "/";

void setup() {
  Serial.begin(9600);

  // Verify WiFi module is available
  while (WiFi.status() == WL_NO_MODULE) {
    Serial.println("Communication with WiFi module failed!");
    delay(2000);
  }

  // Connect to WiFi
  while (status != WL_CONNECTED) {
    char buffer[50];
    sprintf(buffer, "Connecting to network: %s", SSID);
    Serial.println(buffer);

    status = WiFi.begin(SSID, PASS);

    // Give some time for connection to be stablished
    delay(5000);
  }

  Serial.println("Connected!");
}

void loop() {
  Serial.println("\n");
  Serial.println("Making request");
  client.get(PATH_NAME);

  // read the status code and body of the response
  int statusCode = client.responseStatusCode();
  String response = client.responseBody();

  char statusBuffer[30];
  sprintf(statusBuffer, "\nStatus code: %i", statusCode);
  Serial.println(statusBuffer);
  Serial.println("Response: ");
  Serial.println(response);

  Serial.println("\n");

  Serial.println("Waiting...");
  delay(20000);
}
