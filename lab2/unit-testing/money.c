#include <stdlib.h>
#include "money.h"

struct Money {
	double amount;
	char *currency;
};

void money_free(Money *m) {
	free(m);
	return;
}

Money *money_create(double amount, char *currency) {
	Money *m;

	if (amount < 0)
		return NULL;

	m = malloc(sizeof(Money));
	if (m == NULL)
		return NULL;

	m->amount = amount;
	m->currency = currency;

	return m;
}

double money_amount(Money *m) {
	return m->amount;
}

char *money_currency(Money *m) {
	return m->currency;
}

void money_add(Money *m, double amount) {
	m->amount += amount;
}