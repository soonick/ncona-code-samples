FROM ubuntu:20.04

RUN apt-get update -y
RUN apt-get install -y python3-pip python3-dev python3-venv
RUN DEBIAN_FRONTEND=noninteractive TZ=America/New_York apt-get install -y npm

RUN npm i -g nodemon

WORKDIR /app
COPY . /app

RUN python3 -m venv venv
RUN . venv/bin/activate
RUN pip3 install -r requirements.txt

CMD nodemon --exec python3 app.py
