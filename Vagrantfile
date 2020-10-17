# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |c|
  c.vm.box = "bento/ubuntu-20.04"

  #######################
  # VIRTUALBOX SETTINGS #
  #######################

  c.vm.provider "virtualbox" do |v|
    v.cpus   = "4"
    v.gui    = false
    v.memory = "2048"
    v.name   = "elasticlab"

    v.customize ["modifyvm", :id, "--nested-hw-virt", "on"]
    v.customize ["modifyvm", :id, "--paravirtprovider", "kvm"]
    v.customize ["modifyvm", :id, "--vrde", "off"]
  end

  ##################
  # SHARED FOLDERS #
  ##################

  c.vm.synced_folder ".", "/vagrant", disabled: true
  c.vm.synced_folder "./bin", "/home/vagrant/bin"
  c.vm.synced_folder "./infra", "/home/vagrant/.elasticlab/infra"

  ####################
  # HOST INFORMATION #
  ####################

  c.vm.hostname = "elasticlab.local"
  # c.vm.network :private_network, ip: "192.168.56.78"

  c.vm.network :forwarded_port, guest: 22, host: 2222, disabled: true
  c.vm.network :forwarded_port, guest: 22, host: 7878, id: "ssh"

  #############
  # PROVISION #
  #############

  c.vm.provision "shell", inline: "apt-get clean"
  c.vm.provision "shell", inline: "apt-get update"

  c.vm.provision "shell", path: "provision/upgrade.sh"
  c.vm.provision "shell", path: "provision/dependencies.sh"

  c.vm.provision "shell", privileged: false, path: "provision/rust.sh"
  c.vm.provision "shell", path: "provision/terraform.sh"

  # Pre-Build
  c.vm.provision "shell", privileged: false, inline: "cd /home/vagrant/bin && cargo b"
  c.vm.provision "shell", inline: "ln -s /home/vagrant/bin/target/debug/elasticlab /usr/local/bin/el"

  # Change ownership
  c.vm.provision "shell", inline: "chown -R vagrant:vagrant /home/vagrant"
end
