
#include <stdio.h>
#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <vector>
#include <cstdlib>

using namespace std;

vector<string> tokens;

void Tokenize(const string& input) {
    tokens.clear();

    stringstream iss(input);
    string line;

    size_t pos = 0;
    size_t prev = 0;
    while (getline(iss, line)) {
        while ((pos = line.find_first_of(" +-*()", prev)) != string::npos) {

            if (pos >= prev) {
                if (pos != prev)
                    tokens.push_back(line.substr(prev, pos - prev));
                if (line[pos] != ' ') {
                    char op[2] = { line[pos], 0 };
                    tokens.push_back(string(op));
                }
                prev = pos + 1;
            }
        }
        if (prev < line.length())
            tokens.push_back(line.substr(prev, string::npos));
    }

//    for (size_t i = 0; i < tokens.size(); ++i) {
//        cout << tokens[i] << endl;
//    }
}

string ConvertInfixToPostfix() {
    stringstream postfix;
    vector<int> stack;
    string op("+-*");

    for (size_t i = 0; i < tokens.size(); ++i) {
        const string& token = tokens[i];

        size_t idx = op.find_first_of(token, 0);

        if (idx != -1 && token.length() == 1) {
            if (stack.empty()) {
                stack.push_back(idx);
            } else {
                while (!stack.empty()) {
                    int prec2 = stack.back() / 2;
                    int prec1 = idx / 2;
                    if (prec2 > prec1) {
                        postfix << op[stack.back()] << " ";
                        stack.pop_back();
                    } else {
                        break;
                    }
                }
                stack.push_back(idx);
            }
        } else if (token == "(") {
            stack.push_back(-2);
        } else if (token == ")") {
            while (stack.back() != -2) {
                postfix << op[stack.back()] << " ";
                stack.pop_back();
            }
            stack.pop_back();
        } else {
            int number = 0;
            if (istringstream(token) >> number) {
                postfix << token << " ";
            } else {
                cout << "Invalid operator or number!" << endl;
                return string();
            }
        }
    }
    while (!stack.empty()) {
        if (stack.back() == -2) {
            cout << "Parenthesis were mismatched!" << endl;
            return string();
        }
        postfix << op[stack.back()] << " ";
        stack.pop_back();
    }
    return postfix.str();
}

bool RPN(const string& input, int& result) {
    istringstream iss(input);
    vector<int> stack;
    string token;
    while (iss >> token) {
        int number = 0;
        if (istringstream(token) >> number) {
            stack.push_back(number);
        } else {
            if (stack.size() < 2) {
                cout << "Invalid postfix notation!" << endl;
                return false;
            }
            int secondNum = stack.back();
            stack.pop_back();
            int firstNum = stack.back();
            stack.pop_back();

            int newNum = 0;
            if (token == "+") {
                newNum = firstNum + secondNum;
            } else if (token == "-") {
                newNum = firstNum - secondNum;
            } else if (token == "*") {
                newNum = firstNum * secondNum;
            } else {
                cout << "Unsupported operator: " << token << endl;
                exit(0);
            }

            stack.push_back(newNum);
        }
    }
    if (stack.size() != 1) {
        cout << "Invalid postfix notation!" << endl;
        return false;
    }
    result = stack.back();
    return true;
}

int main() {
    cout << "<Calculator>" << endl;

    const char *tests[] = {
        "3 + 4 * 2 - ( 1 - 5 ) + 7",    /* RC mandated: OK */
        "123",                          /* OK */
        "3+4 * 2 * ( 5 - 3 )",          /* OK */
        "(((((((1+2+3*(4 + 5))))))",    /* bad parens */
        "a - (b + c+d * 4)!",           /* unknown op */
        "3 + 4 * 2 ( 1 - 5 ) + 7",      /* invalid */
        "3 + 4 * 2 - * ( 1 - 5 ) + 7",  /* invalid */
        "(1*2)*3 + (10 - 5) *2",        /* OK */
        0
    };

    for (int i = 0; tests[i]; ++i) {
        printf("Testing string '%s'\n", tests[i]);

        Tokenize(string(tests[i]));
        string postfix = ConvertInfixToPostfix();

        if (postfix.empty())
            continue;

        cout << "postfix: " << postfix << endl;

        int result = 0;
        if (RPN(postfix, result)) {
            cout << "result: " << result << endl;
        }
    }

	return 0;
}
