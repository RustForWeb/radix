document.addEventListener('DOMContentLoaded', () => {
    const iframes = document.querySelectorAll('iframe[data-mdbook-trunk]');
    for (const iframe of iframes) {
        if (!(iframe instanceof HTMLIFrameElement)) {
            return;
        }

        iframe.addEventListener('load', () => {
            // TODO: Figure out how to perform this after the iframe size is actually known.
            setTimeout(() => {
                iframe.style.width = `${iframe.contentWindow.document.body.scrollWidth}px`;
                iframe.style.height = `${iframe.contentWindow.document.body.scrollHeight}px`;
            }, 10);
        });
    }
});
