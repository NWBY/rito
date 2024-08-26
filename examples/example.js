class BookCollection {
  constructor() {
    this.books = [];
  }

  addBook(title, author, year) {
    const book = { title, author, year };
    this.books.push(book);
  }

  findBooksByAuthor(author) {
    return this.books.filter(
      (book) => book.author.toLowerCase() === author.toLowerCase()
    );
  }

  getBooksSortedByYear() {
    return [...this.books].sort((a, b) => a.year - b.year);
  }

  toJSON() {
    return JSON.stringify(this.books, null, 2);
  }
}

// Usage
const myBooks = new BookCollection();
myBooks.addBook("The Great Gatsby", "F. Scott Fitzgerald", 1925);
myBooks.addBook("To Kill a Mockingbird", "Harper Lee", 1960);
myBooks.addBook("1984", "George Orwell", 1949);

console.log("Books by Harper Lee:", myBooks.findBooksByAuthor("Harper Lee"));
console.log("\nAll books sorted by year:");
console.log(myBooks.getBooksSortedByYear());
console.log("\nJSON representation:");
console.log(myBooks.toJSON());
