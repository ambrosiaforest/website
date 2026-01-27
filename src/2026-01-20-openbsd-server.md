# how to host a website on openbsd using httpd and relayd

## background
helloo, it's been a while since my last blog post, which was barely even a blog post, so i figured i'd make something real that people will hopefully read...

in this article i'm going to be showing you how to set up a website on an openbsd server using httpd, relayd, and wireguard. so lets get right into it!!

## the setup
i'm going to be showing how i host this in my setup, obviously you are free to do this your own way but to make it easier for me i'm going to be showing my setup. when i was setting up my website, i had a hard time finding good sources on how to run a setup like this. 

my specific setup is i have a server at my home. this server hosts everything like my website, xmpp, mail, etc. i proxy this through wireguard to my vps hosted on [openbsd.amsterdam](https://openbsd.amsterdam). i use their service because they provide very easy to use openbsd vms and they have been very reliable over the nearly 2 years i've been using them. the reason i go with this setup is because i have the freedom of hosting my services on a pc at home while still having all the traffic routed through a vps to prevent my ip from being leaked.

## homeserver setup
since this article is strictly about webservers all we really need to worry about is setting up httpd and wireguard.

### httpd setup
on openbsd, httpd is run in a chroot located at `/var/www`. this means all your webserver files must be within this chroot. i like putting them in the pre made `/var/www/htdocs` directory. meaing the location for my website would be `/var/www/htdocs/example.com`. for this example lets use a very basic html file. so in `/var/www/htdocs/example.com`. put: 

```

