document.addEventListener("DOMContentLoaded", function () {
    const searchInput = document.getElementById("article-search-input");
    const clearBtn = document.getElementById("search-clear-btn");
    const tagWrapper = document.getElementById("tag-buttons-wrapper");
    const container = document.getElementById("article-list-container");
    const noResults = document.getElementById("no-results-message");

    if (!container) return;

    const articles = Array.from(container.querySelectorAll(".article"));
    let selectedTag = "all";

    const tagSet = new Set();
    articles.forEach(function (article) {
        const rawTags = article.getAttribute("data-tags");
        if (rawTags) {
            rawTags.split(",").forEach(function (tag) {
                const trimmed = tag.trim();
                if (trimmed) tagSet.add(trimmed);
            });
        }
    });

    tagSet.forEach(function (tag) {
        const btn = document.createElement("button");
        btn.className = "tag-filter-btn";
        btn.setAttribute("data-tag", tag);
        btn.textContent = tag;
        tagWrapper.appendChild(btn);
    });

    function filterArticles() {
        const query = searchInput ? searchInput.value.trim().toLowerCase() : "";
        let visibleCount = 0;

        articles.forEach(function (article) {
            const title = (article.getAttribute("data-title") || "").toLowerCase();
            const abstract = (article.getAttribute("data-abstract") || "").toLowerCase();
            const rawTags = article.getAttribute("data-tags") || "";
            const tags = rawTags.split(",").map(function (t) { return t.trim(); });

            const matchesQuery = !query || title.includes(query) || abstract.includes(query);
            const matchesTag = selectedTag === "all" || tags.includes(selectedTag);

            if (matchesQuery && matchesTag) {
                article.style.display = "";
                visibleCount++;
            } else {
                article.style.display = "none";
            }
        });

        if (clearBtn) {
            clearBtn.style.display = query ? "inline-block" : "none";
        }

        if (noResults) {
            noResults.style.display = visibleCount === 0 ? "block" : "none";
        }
    }

    if (searchInput) {
        searchInput.addEventListener("input", filterArticles);
    }

    if (clearBtn) {
        clearBtn.addEventListener("click", function () {
            searchInput.value = "";
            filterArticles();
            searchInput.focus();
        });
    }

    if (tagWrapper) {
        tagWrapper.addEventListener("click", function (event) {
            const btn = event.target.closest(".tag-filter-btn");
            if (!btn) return;

            const allBtns = tagWrapper.querySelectorAll(".tag-filter-btn");
            allBtns.forEach(function (b) { b.classList.remove("active"); });
            btn.classList.add("active");

            selectedTag = btn.getAttribute("data-tag") || "all";
            filterArticles();
        });
    }
});
