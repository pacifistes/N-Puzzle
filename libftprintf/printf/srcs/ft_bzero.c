/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_bzero.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/03/29 18:28:19 by bbrunell          #+#    #+#             */
/*   Updated: 2016/03/29 18:28:25 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "ft_printf.h"

void	ft_bzero(void *f, size_t i)
{
	char	*str;
	size_t	j;

	j = 0;
	str = (char *)f;
	while (j < i)
	{
		str[j++] = 0;
	}
}