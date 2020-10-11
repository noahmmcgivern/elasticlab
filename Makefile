make:
	make up snap

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
