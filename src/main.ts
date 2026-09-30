import { invoke } from "@tauri-apps/api/core";

type Question = {
  id: string;
  asked_at: string;
  asker: string;
  context: string;
  question: string;
  tags: string[];
  draft: unknown | null;
};

window.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector<HTMLFormElement>("#question-form");
  const questionInput =
    document.querySelector<HTMLTextAreaElement>("#question");
  const askerInput = document.querySelector<HTMLInputElement>("#asker");
  const contextInput = document.querySelector<HTMLInputElement>("#context");
  const tagsInput = document.querySelector<HTMLInputElement>("#tags");
  const questionList = document.querySelector<HTMLDivElement>("#question-list");
  const errorMessage =
    document.querySelector<HTMLParagraphElement>("#error-message");

  if (
    !form ||
    !questionInput ||
    !askerInput ||
    !contextInput ||
    !tagsInput ||
    !questionList ||
    !errorMessage
  ) {
    return;
  }

  function showError(message: string) {
    errorMessage.textContent = message;
    errorMessage.hidden = false;
  }

  function clearError() {
    errorMessage.textContent = "";
    errorMessage.hidden = true;
  }

  function renderQuestions(questions: Question[]) {
    questionList.innerHTML = "";

    if (questions.length === 0) {
      questionList.innerHTML =
        '<p class="empty-state">No questions saved yet.</p>';
      return;
    }

    questions.forEach((item) => {
      const card = document.createElement("article");
      card.className = "question-card";

      const question = document.createElement("h3");
      question.textContent = item.question;

      const asker = document.createElement("p");
      asker.textContent = item.asker
        ? `Asker: ${item.asker}`
        : "Asker: Not provided";

      const context = document.createElement("p");
      context.textContent = item.context
        ? `Context: ${item.context}`
        : "Context: Not provided";

      const date = document.createElement("p");
      date.textContent = `Date: ${new Date(item.asked_at).toLocaleString()}`;

      const tags = document.createElement("p");
      tags.textContent =
        item.tags.length > 0
          ? `Tags: ${item.tags.join(", ")}`
          : "Tags: None";

      const deleteButton = document.createElement("button");
      deleteButton.type = "button";
      deleteButton.textContent = "Delete";
      deleteButton.className = "delete-button";

      deleteButton.addEventListener("click", async () => {
        const confirmed = window.confirm(
          "Are you sure you want to delete this question?"
        );

        if (!confirmed) {
          return;
        }

        try {
          clearError();

          await invoke("delete_question", {
            id: item.id,
          });

          await loadQuestions();
        } catch (error) {
          showError(String(error));
        }
      });

      card.appendChild(question);
      card.appendChild(asker);
      card.appendChild(context);
      card.appendChild(date);
      card.appendChild(tags);
      card.appendChild(deleteButton);

      questionList.appendChild(card);
    });
  }

  async function loadQuestions() {
    try {
      clearError();

      const questions = await invoke<Question[]>("list_questions");

      renderQuestions(questions);
    } catch (error) {
      showError(`Could not load questions: ${String(error)}`);
    }
  }

  form.addEventListener("submit", async (event) => {
    event.preventDefault();

    clearError();

    const question = questionInput.value.trim();

    if (!question) {
      showError("Question is required.");
      return;
    }

    const tags = tagsInput.value
      .split(",")
      .map((tag) => tag.trim())
      .filter((tag) => tag.length > 0);

    try {
      await invoke("save_question", {
        asker: askerInput.value.trim(),
        context: contextInput.value.trim(),
        question,
        tags,
      });

      form.reset();

      await loadQuestions();
    } catch (error) {
      showError(`Could not save question: ${String(error)}`);
    }
  });

  loadQuestions();
});