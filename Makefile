prod:
	make build run

dev:
	make up snap

build:
	docker build -t elasticlab --rm .

run:
	docker run -it --name elasticlab --rm elasticlab

up:
	vagrant up

d:
	vagrant snapshot delete NEW; vagrant destroy -f

ssh:
	vagrant ssh

sus:
	vagrant suspend

halt:
	vagrant halt

snap:
	vagrant snapshot save NEW || exit 0

res:
	vagrant snapshot restore NEW
