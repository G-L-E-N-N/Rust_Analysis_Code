#include <iostream>

class Node {
public:
    int value;
    Node* next;  
    Node(int val) : value(val), next(nullptr) {}
};

class CyclicList {
public:
    Node* head;  
    CyclicList() : head(nullptr) {}
    
    void addNode(int val) {
        Node* newNode = new Node(val);
        if (head == nullptr) {
            head = newNode;
            newNode->next = head;  // Create cycle
        } else {
            Node* temp = head;
            while (temp->next != head) {
                temp = temp->next;
            }
            temp->next = newNode;
            newNode->next = head;  // Close cycle
        }
    }
};

int main() {
    CyclicList list;

    list.addNode(1);
    list.addNode(2);
    list.addNode(3);

    Node* current = list.head;
    if (current != nullptr) {
        do {
            std::cout << current->value << " ";
            current = current->next;
        } while (current != list.head);
    }
    std::cout << std::endl;

    return 0;
}
