document.addEventListener("DOMContentLoaded", function () {
    // 1. Fetch and display books from the backend
    function fetchBooks() {
        fetch("/books")
            .then(response => response.json())
            .then(data => {
                const booksContainer = document.getElementById("books");
                booksContainer.innerHTML = '';
                data.forEach(book => {
                    const bookElement = document.createElement("div");
                    bookElement.classList.add("book");
                    bookElement.innerHTML = `
                        <h3>${book.title}</h3>
                        <p>Author: ${book.author}</p>
                        <p>Price: $${book.price}</p>
                    `;
                    booksContainer.appendChild(bookElement);
                });
            });
    }

    // Fetch books when page loads
    fetchBooks();

    // 2. Handle the add book form submission
    const bookForm = document.getElementById("book-form");
    bookForm.addEventListener("submit", function (event) {
        event.preventDefault();

        const title = document.getElementById("book-title").value;
        const author = document.getElementById("book-author").value;
        const price = parseFloat(document.getElementById("book-price").value);

        fetch("/add", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ title, author, price })
        })
            .then(response => response.json())
            .then(data => {
                alert("Book added successfully!");
                fetchBooks(); // Refresh the book list
            });
    });
});
