RU=rustc
OBJ=$(SRC:.rs=.o)

PREFIX=/usr/bin

%.o: %.rs
	$(RU) $@ $<


foklang-shell: $(OBJ)
	$(RU) shell.rs $(OBJ) -o foklang-shell


.PHONY: clean
clean:
	rm -f $(OBJ) foklang-shell

.PHONY: install
install: foklang-shell
	mkdir -p $(DESTDIR)$(PREFIX)
	cp $< $(DESTDIR)$(PREFIX)/foklang-shell

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(PREFIX)/foklang-shell
