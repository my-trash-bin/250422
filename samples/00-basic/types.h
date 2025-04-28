#ifndef TYPES_H
#define TYPES_H

typedef struct Shape Shape;
typedef struct Circle Circle;
typedef struct Square Square;

typedef enum __Shape_type {
    __Shape_type_Circle,
    __Shape_type_Square,
} Shape_type;

struct Circle {
    float radius;
};

struct Square {
    float size;
};

struct Shape {
    Shape_type type;
    union {
        Circle circle;
        Square square;
    } value;
};

#endif
