//scroll nav style
window.addEventListener('scroll', () => {
  document.querySelector('.header__container').classList.toggle('window-scrolled', window.scrollY > 0);
  document.querySelector('header').classList.toggle('hide-content', window.scrollY > 0)
})


//Gallery images populated with ajax request
var folder = "gallery";

function loadGalleryImages() {
  return $.ajax({
    url: folder,
    success: function (data) {
      $(data).find("a").attr("href", function (i, val) {
        if (val.match(/.(jpe?g|png|gif)$/)) {
          $("ul.image-gallery").append("<li><img src='" + val + "'></li>");
        }
      });
    }
  });
}


//Modal Gallery previews must be added only after ajax request completes
loadGalleryImages().then(function () {
  var img = document.querySelectorAll(".image-gallery li img");
  var modal = document.getElementById("gallery-modal");
  var modalImg = document.getElementById("gallery-modal-img");
  var captionText = document.getElementById("gallery-modal-caption");

  img.forEach((imgA) => {
    imgA.onclick = function () {
      modal.style.display = "block";
      modalImg.src = this.src;
      captionText.innerHTML = this.alt;
    }


    modal.onclick = function () {
      modal.style.display = "none"
    }
  });
});

//Hamburger menu for small screens
if (window.innerWidth < 1024) {


  const nav = document.querySelector('.nav__links');
  const openNavBtn = document.querySelector('#nav__toggle-open');
  const closeNavBtn = document.querySelector('#nav__toggle-close');

  const openNav = () => {
    nav.style.display = 'flex';
    openNavBtn.style.display = 'none';
    closeNavBtn.style.display = 'inline-block';
  }

  openNavBtn.addEventListener('click', openNav);

  const closeNav = () => {
    nav.style.display = 'none';
    openNavBtn.style.display = 'inline-block';
    closeNavBtn.style.display = 'none';
  }

  closeNavBtn.addEventListener('click', closeNav);

  nav.querySelectorAll('li a').forEach(navLink => {
    navLink.addEventListener('click', closeNav);
  });

}