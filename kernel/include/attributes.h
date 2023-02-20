#ifndef HELIUM_ATTRIBUTES_H
#define HELIUM_ATTRIBUTES_H

/**
 * \brief Marks a struct or variable as packed.
 * \note Equivalent to GCC's __attribute__((packed)).
 */
#define pack __attribute__((packed))

#define AS_STRING_IMPL(x) #x
#define AS_STRING(x) AS_STRING_IMPL(x)

#endif