document.addEventListener("DOMContentLoaded", function () {
    // 检查登录状态
    const user = checkLoginStatus();
    
    // 设置登出按钮
    setupLogout();
    
    // DOM elements
    const booksContainer = document.getElementById("books-container");
    const bookForm = document.getElementById("book-form");
    const searchInput = document.getElementById("search-input");
    const searchButton = document.getElementById("search-button");
    const categoryFilters = document.querySelectorAll(".category-filter");
    
    let allBooks = []; // Store all books for filtering
    
    // 1. Fetch and display books from the backend
    function fetchBooks() {
        fetch("/api/books")
            .then(response => {
                if (!response.ok) {
                    throw new Error("Network response was not ok");
                }
                return response.json();
            })
            .then(data => {
                allBooks = data;
                displayBooks(data);
            })
            .catch(error => {
                console.error("Error fetching books:", error);
                displayError("无法加载图书，请稍后再试。");
            });
    }
    
    // Display books in the UI
    function displayBooks(books) {
        if (!booksContainer) return;
        
        booksContainer.innerHTML = '';  // Clear the container before adding new books
        
        if (books.length === 0) {
            booksContainer.innerHTML = '<p class="no-books">没有找到符合条件的图书</p>';
            return;
        }
        
        books.forEach(book => {
            const bookElement = document.createElement("div");
            bookElement.classList.add("book-card");
            
            // Set default image if none provided
            const imageUrl = book.image || "/assets/images/book-default.jpg";
            
            bookElement.innerHTML = `
                <img src="${imageUrl}" alt="${book.title}" class="book-image">
                <div class="book-info">
                    <h3 class="book-title">${book.title}</h3>
                    <p class="book-author">作者: ${book.author}</p>
                    <p class="book-price">价格: ¥${book.price.toFixed(2)}</p>
                    ${book.description ? `<p class="book-description">${book.description}</p>` : ''}
                    <button class="action-button view-details" data-id="${book.id}">查看详情</button>
                    <button class="action-button add-to-cart" data-id="${book.id}">加入购物车</button>
                </div>
            `;
            
            booksContainer.appendChild(bookElement);
        });
        
        // Add event listeners to buttons
        addBookButtonListeners();
    }
    
    // Add event listeners to book buttons
    function addBookButtonListeners() {
        // View details buttons
        document.querySelectorAll(".view-details").forEach(button => {
            button.addEventListener("click", function() {
                const bookId = this.getAttribute("data-id");
                viewBookDetails(bookId);
            });
        });
        
        // Add to cart buttons
        document.querySelectorAll(".add-to-cart").forEach(button => {
            button.addEventListener("click", function() {
                const bookId = this.getAttribute("data-id");
                addToCart(bookId);
            });
        });
    }
    
    // View book details
    function viewBookDetails(bookId) {
        fetch(`/api/books/${bookId}`)
            .then(response => {
                if (!response.ok) {
                    throw new Error("Book not found");
                }
                return response.json();
            })
            .then(book => {
                showBookModal(book);
            })
            .catch(error => {
                console.error("Error fetching book details:", error);
                displayError("无法加载图书详情");
            });
    }
    
    // Show book modal with details
    function showBookModal(book) {
        // Create modal element
        const modal = document.createElement("div");
        modal.classList.add("modal");
        
        modal.innerHTML = `
            <div class="modal-content">
                <span class="close-modal">&times;</span>
                <div class="book-details">
                    <div class="book-details-image">
                        <img src="${book.image || "/assets/images/book-default.jpg"}" alt="${book.title}">
                    </div>
                    <div class="book-details-info">
                        <h2>${book.title}</h2>
                        <p class="book-author">作者: ${book.author}</p>
                        <p class="book-price">价格: ¥${book.price.toFixed(2)}</p>
                        ${book.category ? `<p class="book-category">分类: ${book.category}</p>` : ''}
                        ${book.description ? `<p class="book-description">${book.description}</p>` : ''}
                        <button class="action-button add-to-cart" data-id="${book.id}">加入购物车</button>
                    </div>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        // Close modal when clicking on X or outside the modal
        const closeBtn = modal.querySelector(".close-modal");
        closeBtn.addEventListener("click", () => {
            document.body.removeChild(modal);
        });
        
        window.addEventListener("click", (event) => {
            if (event.target === modal) {
                document.body.removeChild(modal);
            }
        });
        
        // Add to cart button in modal
        const addToCartBtn = modal.querySelector(".add-to-cart");
        addToCartBtn.addEventListener("click", () => {
            addToCart(book.id);
        });
    }
    
    // Add book to cart
    function addToCart(bookId) {
        // Get cart from localStorage or initialize empty array
        let cart = JSON.parse(localStorage.getItem("cart")) || [];
        
        // Check if book is already in cart
        const bookInCart = cart.find(item => item.id === bookId);
        
        if (bookInCart) {
            bookInCart.quantity += 1;
            showNotification("已增加图书数量！");
        } else {
            // Find book in allBooks array
            const bookToAdd = allBooks.find(book => book.id === parseInt(bookId) || book.id === bookId);
            
            if (bookToAdd) {
                cart.push({
                    id: bookToAdd.id,
                    title: bookToAdd.title,
                    author: bookToAdd.author,
                    price: bookToAdd.price,
                    image: bookToAdd.image || "/assets/images/book-default.jpg",
                    quantity: 1
                });
                showNotification("图书已添加到购物车！");
            }
        }
        
        // Save updated cart to localStorage
        localStorage.setItem("cart", JSON.stringify(cart));
        
        // Update cart count
        updateCartCount();
    }
    
    // Update cart count in navbar
    function updateCartCount() {
        const cartCountElement = document.querySelector(".cart-count");
        if (!cartCountElement) return;
        
        const cart = JSON.parse(localStorage.getItem("cart")) || [];
        const totalItems = cart.reduce((total, item) => total + item.quantity, 0);
        
        cartCountElement.textContent = totalItems;
        
        if (totalItems > 0) {
            cartCountElement.style.display = "flex";
            // 添加闪烁动画效果
            cartCountElement.classList.add('pulse');
            setTimeout(() => {
                cartCountElement.classList.remove('pulse');
            }, 1000);
        } else {
            cartCountElement.style.display = "none";
        }
    }
    
    // Show notification with type
    function showNotification(message, type = "success") {
        const notification = document.createElement("div");
        notification.classList.add("notification", type);
        notification.textContent = message;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.classList.add("show");
        }, 10);
        
        setTimeout(() => {
            notification.classList.remove("show");
            setTimeout(() => {
                document.body.removeChild(notification);
            }, 300);
        }, 3000);
    }
    
    // Display error message
    function displayError(message) {
        showNotification(message);
    }
    
    // 2. Handle the add book form submission
    if (bookForm) {
        bookForm.addEventListener("submit", function (event) {
            event.preventDefault();
            
            const title = document.getElementById("book-title").value;
            const author = document.getElementById("book-author").value;
            const price = parseFloat(document.getElementById("book-price").value);
            const description = document.getElementById("book-description")?.value || null;
            const category = document.getElementById("book-category")?.value || null;
            
            fetch("/api/books", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ title, author, price, description, category })
            })
                .then(response => {
                    if (!response.ok) {
                        throw new Error("Failed to add book");
                    }
                    return response.json();
                })
                .then(data => {
                    showNotification("图书添加成功!");
                    bookForm.reset();
                    fetchBooks(); // Refresh the book list
                })
                .catch(error => {
                    console.error("Error adding book:", error);
                    displayError("添加图书失败，请稍后再试");
                });
        });
    }
    
    // 3. Search functionality
    if (searchButton && searchInput) {
        searchButton.addEventListener("click", function() {
            searchBooks();
        });
        
        searchInput.addEventListener("keypress", function(event) {
            if (event.key === "Enter") {
                searchBooks();
            }
        });
    }
    
    function searchBooks() {
        const searchTerm = searchInput.value.toLowerCase().trim();
        
        if (searchTerm === "") {
            displayBooks(allBooks);
            showNotification("请输入搜索关键词", "warning");
            return;
        }
        
        // 跳转到搜索结果页面
        window.location.href = `/search.html?q=${encodeURIComponent(searchTerm)}`;
    }
    
    // 4. Category filtering
    if (categoryFilters) {
        categoryFilters.forEach(filter => {
            filter.addEventListener("click", function() {
                const category = this.getAttribute("data-category");
                
                // Remove active class from all filters
                categoryFilters.forEach(f => f.classList.remove("active"));
                
                // Add active class to clicked filter
                this.classList.add("active");
                
                if (category === "all") {
                    displayBooks(allBooks);
                } else {
                    const filteredBooks = allBooks.filter(book => 
                        book.category && book.category.toLowerCase() === category.toLowerCase()
                    );
                    displayBooks(filteredBooks);
                }
            });
        });
    }
    
    // Initialize the app
    fetchBooks();
    updateCartCount();
});

// 用户登录状态管理
function checkLoginStatus() {
    const userJson = localStorage.getItem('user');
    if (userJson) {
        try {
            const user = JSON.parse(userJson);
            // 更新UI显示用户已登录
            document.querySelector('.login-item').style.display = 'none';
            document.querySelector('.user-menu').style.display = 'block';
            
            // 设置用户名显示
            document.getElementById('username-display').textContent = user.username;
            
            // 如果是学生，显示学生标记
            if (user.is_student) {
                document.getElementById('student-badge').style.display = 'inline-block';
            } else {
                document.getElementById('student-badge').style.display = 'none';
            }
            
            return user;
        } catch (e) {
            console.error('解析用户数据出错:', e);
            localStorage.removeItem('user');
        }
    }
    
    // 未登录状态
    document.querySelector('.login-item').style.display = 'block';
    document.querySelector('.user-menu').style.display = 'none';
    return null;
}

// 处理登出
function setupLogout() {
    const logoutBtn = document.getElementById('logout-btn');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', function(e) {
            e.preventDefault();
            
            // 清除用户数据
            localStorage.removeItem('user');
            
            // 更新UI
            checkLoginStatus();
            
            // 显示通知
            showNotification('您已成功退出登录');
        });
    }
}
