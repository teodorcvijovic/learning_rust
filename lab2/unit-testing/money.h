#ifndef MONEY_H
#define MONEY_H

typedef struct Money Money;

Money *money_create(double amount, char *currency);
void money_free(Money *m);

double money_amount(Money *m);
char *money_currency(Money *m);

void money_add(Money *m, double amount);

#endif /* MONEY_H */