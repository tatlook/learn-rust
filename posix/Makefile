root:
	cc gainroot-DANGEROUS.c	
	sudo chown root a.out
	sudo chmod +s a.out
	./a.out


# Windows-alike file name, just to help the gitignore file.
sock:
	cc sockserver.c -o a.exe
	cc sockclient.c -o b.exe
	./a.exe & sh -c './b.exe & ./b.exe & ./b.exe & ./b.exe & ./b.exe &\
		./b.exe & ./b.exe & ./b.exe & ./b.exe & ./b.exe;\
		wait; echo "All clients finished"';\
		wait
# inexplicable behaviour: "All clients finished" is printed before the clients finish
