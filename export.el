(require 'package)
(require 'org)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/"))
(package-initialize)
(unless package-archive-contents
 (package-refresh-contents))
(package-install 'f)
(package-install 'ox-gfm)
(package-vc-install '(ox-gemini . (:url "https://labs.phundrak.com/phundrak/ox-gemini.git")))

(require 'f)
(require 'ox-gfm)
(require 'ox-gemini)
(require 'ox-publish)

(setq org-confirm-babel-evaluate nil
      org-html-validation-link nil
      make-backup-files nil
      org-export-with-broken-links t)

(defvar project-root
  (file-name-as-directory (expand-file-name "content" default-directory)))

(dolist (file (f-files project-root (lambda (file) (f-ext-p file "org")) t))
  (message "====== Exporting %s" (f-relative file))
  (with-temp-buffer
      (find-file file)
      (org-export-to-file 'gfm (concat (f-no-ext file) ".md"))
      (org-export-to-file 'gemini (concat (f-no-ext file) ".gmi"))))

(defvar gemini-root-dir "gemini")
(dolist (subdir '("" "en" "lfn"))
  (f-mkdir (f-join gemini-root-dir subdir)))
(f-mkdir gemini-root-dir)
(dolist (file (mapcar #'f-relative (f-files "." (lambda (file) (f-ext-p file "gmi")) t)))
  (let ((new-place (f-join gemini-root-dir (apply #'f-join (cdr (f-split file))))))
    (f-move file new-place)))

(message "Project generated!")
