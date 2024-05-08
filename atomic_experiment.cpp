#include <iostream>
#include <atomic>
#include <cstdint>
#include <cstring> // For memcpy

int main() {
    // Define a byte array
    alignas(4) unsigned char data[4] = {};

    // Ensure the pointer is properly aligned and treat it as an atomic int32_t
    std::atomic<int32_t>* p = reinterpret_cast<std::atomic<int32_t>*>(data);

    // New value to store atomically
    int32_t new_val = 123456;

    // Perform an atomic store
    p->store(new_val, std::memory_order_release);

    // For demonstration: print the stored value by copying it back to an int32_t
    int32_t stored_val;
    memcpy(&stored_val, data, sizeof(stored_val));
    std::cout << "Stored value: " << stored_val << std::endl;

    return 0;
}
