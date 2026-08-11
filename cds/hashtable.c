#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

struct Node {
    struct Node *next;
    char *key;
    int value;
};

#define SIZE 255

struct HashTable {
    struct Node *table[SIZE];
};

unsigned int hash_string(const char *s) {
    unsigned int h = 0;
    while (*s) {
        h += *s % 32;
        s++;
    }
    return h % SIZE;
}

int get(const struct HashTable *ht, const char *key) {
    int index = hash_string(key);
    const struct Node *node = ht->table[index];
    while (node) {
        if (!strcmp(key, node->key)) {
            return node->value;
        }
        node = node->next;
    }
    fprintf(stderr, "get(): no key: %s", key);
    return 0;
}

void delete(struct HashTable *ht, const char *key) {
    int index = hash_string(key);
    struct Node *node = ht->table[index];
    struct Node *prev = NULL;

    while (node) {
        if (!strcmp(key, node->key)) {
            if (prev) {
                prev->next = node->next;
            } else {
                ht->table[index] = node->next;
            }
            free(node->key);
            free(node);
            return;
        }
        prev = node;
        node = node->next;
    }
    fprintf(stderr, "delete(): no key: %s", key);
}

void insert(struct HashTable *ht, const char *key, int value) {
    int index = hash_string(key);
    struct Node *node = malloc(sizeof(struct Node));
    node->value = value;
    node->key = strdup(key); // FIXME: insert two same key, get repeated
    node->next = ht->table[index];
    ht->table[index] = node;
}

void init(struct HashTable *ht) {
    memset(ht, 0, sizeof(struct HashTable));
}

void print_all(const struct HashTable *ht) {
    for (int i = 0; i < SIZE; i++) {
        const struct Node *node = ht->table[i];
        while (node) {
            printf("\"%s\"=%d  ", node->key, node->value);
            node = node->next;
        }
    }
    putchar('\n');
}

int main() {
    char *a = "Hello Ye Sir";
    char *b = "World We Must";
    printf("%s(%d), %s(%d)", a, hash_string(a), b, hash_string(b));

    struct HashTable ht;
    init(&ht);
    insert(&ht, "void", 12);
    insert(&ht, "char", -12000);
    
    insert(&ht, "int", 100);
    insert(&ht, "tin", 16); // of same hashcode
    insert(&ht, "nit", 16); // of same hashcode

    printf("%d %d %d %d\n", get(&ht, "void"), 
            get(&ht, "int"), get(&ht, "tin"), get(&ht, "char"));
    print_all(&ht);

    delete(&ht, "void");
    print_all(&ht);
    delete(&ht, "tin");
    print_all(&ht);
    delete(&ht, "nit");
    print_all(&ht);

    return 0;
}
