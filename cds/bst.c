#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

struct Node {
    struct Node *left;
    struct Node *right;
    char *value;
};

struct Tree {
    struct Node *root;
};

static struct Node *_insert_node(struct Node *root, const char *value) {
    if (root == NULL) {
        struct Node *node = malloc(sizeof(struct Node));
        node->value = strdup(value);
        node->left = node->right = NULL;
        return node;
    }

    int cmp = strcmp(root->value, value);
    if (cmp > 0) {
        root->left = _insert_node(root->left, value);
    } else if (cmp < 0) {
        root->right = _insert_node(root->right, value);
    } else /* equal */ {
        /* Nothing, do not add repeated values. */
    }
    return root;
}

void insert(struct Tree *tree, const char *value) {
    tree->root = _insert_node(tree->root, value);
}

static struct Node *_adopt(struct Node *root, struct Node *node) {
    if (root == NULL) {
        return node;
    }
    if (node == NULL) {
        return root;
    }
    // caller _delete() or _adopt() assured the node is greater than root
    // and there shall not have been any equal nodes
    assert(strcmp(root->value, node->value) < 0);
    
    root->right = _adopt(root->right, node);
    return root;
}

static struct Node *_delete(struct Node *node, const char *value) {
    if (node == NULL) {
        return NULL;
    }
    int cmp = strcmp(node->value, value);
    if (cmp > 0) {
        node->left = _delete(node->left, value);
        return node;
    } else if (cmp < 0) {
        node->right = _delete(node->right, value);
        return node;
    } else /* equal */ {
        struct Node *left = node ->left;
        struct Node *right = node ->right;
        free(node->value);
        free(node);
        return _adopt(left, right);
        /* Why not implement it as a loop?
         * The answer: Too used to write recrusive code when handling tree.
         */
    }
}

/* delete, _delete and _adopt are my own creation!
 * quite obious though. */
void delete(struct Tree *tree, const char *value) {
    tree->root = _delete(tree->root, value);
}

static void _print_node(const struct Node *node, int depth) {
    if (node) {
        _print_node(node->left, depth + 1);
        for (int i = 0; i < depth; i++) putchar(' ');
        printf("%s\n", node->value);
        _print_node(node->right, depth + 1);
    }
}

void print_all_in_order_pretty(const struct Tree *tree) {
    puts("\n"); // two new lines
    _print_node(tree->root, 0);
}

void init(struct Tree *tree) {
    tree->root = NULL;
}

int main() {
    struct Tree t;
    init(&t);
    /* Unix-like operating systems listed in beginner-friendly to HARD-CORE order.
     * (Not very accurate though)
     * The ones on the list are mostly if not purely consisted of FREE softwares. */
    insert(&t, "Linux Mint");
    insert(&t, "Zorin OS");
    insert(&t, "Pop!_OS");
    insert(&t, "Ubuntu");
    insert(&t, "Debian GNU/Linux");
    insert(&t, "Fedora");
    insert(&t, "GhostBSD");
    insert(&t, "Tiny Core Linux");
    // From here on command line are obligatory
    insert(&t, "FreeBSD");
    insert(&t, "Arch Linux");
    insert(&t, "DragonFlyBSD");
    insert(&t, "Void Linux");
    insert(&t, "NetBSD");
    insert(&t, "Gentoo");
    insert(&t, "Hyperbola GNU/Linux-libre");
    insert(&t, "Alpine Linux");
    insert(&t, "OpenBSD");
    // Functional programming involved, learning curve high
    insert(&t, "NixOS");
    insert(&t, "Guix");
    // These are a bit abandoned/never existed so may be extremly hard to install/use.
    insert(&t, "Debian GNU/kFreeBSD");
    insert(&t, "Debian GNU/kOpenBSD");
    insert(&t, "Debian GNU/Minix");
    insert(&t, "Debian GNU/Hurd");
    insert(&t, "PureDarwin");
    // THE GOAT
    insert(&t, "LFS");
    print_all_in_order_pretty(&t);


    delete(&t, "Linux Mint"); // the root!
    delete(&t, "Fedora");     /* and then just delete some random things */
    delete(&t, "Void Linux"); /* no political reasons involved */
    delete(&t, "Gentoo");
    delete(&t, "nonexistence"); /* also test this */
    print_all_in_order_pretty(&t);
}