# Design Review: Domain Model Refactoring

## Problem/Feature Description

You are reviewing the domain model for a library management system. The development team has been struggling with business logic scattered across services, difficulty testing business rules, and frequent bugs in book lending policies. The current model has grown organically and now exhibits classic "anemic domain model" symptoms.

Your task is to evaluate the current domain design and provide recommendations to transform it into a rich domain model that properly encapsulates business logic and rules.

## Output Specification

Create a design review report called `domain-refactor.md` that:

1. **Classifies** the design decision type (architectural, tactical, or foundational)
2. **Evaluates entity design** - identify anemic vs rich entity patterns and violations
3. **Assesses business logic placement** - determine where domain rules should live
4. **Provides refactoring recommendations** using BAD/GOOD format
5. **Documents the tradeoffs** of moving to rich domain entities

Focus specifically on entity design patterns and domain modeling principles.

## Input Files

=============== FILE: book.ts ===============
export interface Book {
  id: string;
  title: string;
  author: string;
  isbn: string;
  availableCopies: number;
  totalCopies: number;
  category: string;
  publishedYear: number;
}

=============== FILE: member.ts ===============
export interface Member {
  id: string;
  name: string;
  email: string;
  membershipType: 'STUDENT' | 'FACULTY' | 'PUBLIC';
  joinDate: Date;
  currentLoans: string[]; // book IDs
  overdueCount: number;
}

=============== FILE: loan-service.ts ===============
export class LoanService {
  async borrowBook(memberId: string, bookId: string): Promise\<Loan> {
    const member = await this.memberRepo.findById(memberId);
    const book = await this.bookRepo.findById(bookId);

    // Business rules scattered in service
    if (member.membershipType === 'STUDENT' && member.currentLoans.length >= 3) {
      throw new Error('Students can only borrow 3 books');
    }
    
    if (member.membershipType === 'FACULTY' && member.currentLoans.length >= 10) {
      throw new Error('Faculty can only borrow 10 books');
    }
    
    if (member.membershipType === 'PUBLIC' && member.currentLoans.length >= 2) {
      throw new Error('Public members can only borrow 2 books');
    }

    if (member.overdueCount > 2) {
      throw new Error('Cannot borrow with overdue books');
    }

    if (book.availableCopies <= 0) {
      throw new Error('Book not available');
    }

    // Update data directly
    book.availableCopies--;
    member.currentLoans.push(bookId);

    const loan = {
      id: generateId(),
      memberId,
      bookId,
      borrowDate: new Date(),
      dueDate: this.calculateDueDate(member.membershipType),
      status: 'ACTIVE'
    };

    await this.loanRepo.save(loan);
    await this.bookRepo.save(book);
    await this.memberRepo.save(member);

    return loan;
  }

  private calculateDueDate(memberType: string): Date {
    const days = memberType === 'STUDENT' ? 14 : memberType === 'FACULTY' ? 30 : 7;
    const dueDate = new Date();
    dueDate.setDate(dueDate.getDate() + days);
    return dueDate;
  }
}
